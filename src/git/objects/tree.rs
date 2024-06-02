use crate::git::git_fs::{get_blob_object, get_tree_object};
use crate::git::objects::blob::BlobObject;
use crate::git::objects::header::{ObjectHeader, ObjectType};
use crate::git::traits::{Hash, ObjectSave, ToBytes};
use anyhow::{bail, Error, Result};
use std::path::Path;

#[derive(Debug)]
pub struct TreeObject {
    pub header: ObjectHeader,
    pub entries: Vec<TreeEntry>,
}

#[derive(Debug, Clone)]
pub struct TreeEntry {
    pub mode: u32,
    pub name: String,
    pub hash: String,
}

impl TreeObject {
    pub fn write_tree_object(path: &Path) -> Result<TreeObject> {
        if !path.exists() || path.is_file() {
            bail!("Invalid path");
        }

        // read the directory
        let mut entries = Vec::new();

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;

            // if it's .git directory, skip it
            let path = entry.path();
            let path = path.to_str().unwrap();
            if path.contains(".git") {
                continue;
            }

            let entry_path = entry.path();
            let entry_name = entry_path.file_name().unwrap().to_str().unwrap();

            let metadata = entry.metadata()?;

            let mode = if metadata.is_dir() { 40000 } else { 100644 };

            if metadata.is_dir() {
                let tree_object = Self::write_tree_object(entry_path.as_path())?;

                entries.push(TreeEntry {
                    mode,
                    name: entry_name.to_string(),
                    hash: tree_object.hash(),
                });
            } else {
                let blob_object = BlobObject::write_blob_object(entry_path.as_path())?;

                entries.push(TreeEntry {
                    mode,
                    name: entry_name.to_string(),
                    hash: blob_object.hash(),
                });
            };
        }

        let tree = TreeObject {
            header: ObjectHeader {
                object_type: ObjectType::Tree,
                size: entries.iter().map(|x| x.len()).sum(),
            },
            entries,
        };

        let _ = tree.save_object()?;

        Ok(tree)
    }

    // return the vector of (path, data)
    #[async_recursion::async_recursion]
    pub async fn get_files_recursive(&self, prefix: &str) -> Result<Vec<(String, Vec<u8>)>> {
        let mut files = Vec::new();

        for entry in &self.entries {
            // println!("{:?}", entry);
            if entry.mode == 40000 {
                let tree = get_tree_object(&entry.hash)?;

                files.extend(
                    tree.get_files_recursive(&format!("{}/{}", prefix, entry.name))
                        .await?,
                );
            } else {
                let blob = get_blob_object(&entry.hash)?;

                files.push((format!("{}/{}", prefix, entry.name), blob.data));
            }
        }

        Ok(files)
    }
}

impl ToBytes for TreeObject {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(self.header.to_bytes().as_slice());

        let mut entries = self.entries.clone();
        entries.sort_by(|a, b| a.name.cmp(&b.name));

        for entry in entries {
            result.extend_from_slice(entry.to_bytes().as_slice());
        }

        result
    }
}

impl TryFrom<Vec<u8>> for TreeEntry {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self> {
        let space_index = value.iter().position(|&x| x == b' ').unwrap();
        let null_index = value.iter().position(|&x| x == 0).unwrap();

        let mode = std::str::from_utf8(&value[..space_index])?.parse()?;
        let name = std::str::from_utf8(&value[space_index + 1..null_index])?.to_string();
        let hash = hex::encode(&value[null_index + 1..null_index + 21]);

        Ok(TreeEntry { mode, name, hash })
    }
}

impl TreeEntry {
    pub fn len(&self) -> usize {
        self.to_bytes().len()
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
}

impl ToBytes for TreeEntry {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(format!("{} {}\0", self.mode, self.name).as_bytes());
        result.extend_from_slice(&hex::decode(&self.hash).unwrap());
        result
    }
}

impl TryFrom<Vec<u8>> for TreeObject {
    type Error = Error;

    fn try_from(data: Vec<u8>) -> Result<Self> {
        let header = ObjectHeader::try_from(data.clone())?;
        let header_size = header.header_size();

        let mut entries = Vec::new();

        let mut i = header_size;

        while i < data.len() {
            let entry = TreeEntry::try_from(data[i..].to_vec())?;
            i += entry.len();
            entries.push(entry);
        }

        Ok(TreeObject { header, entries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_entry_from() {
        let data = b"100644 file.txt\0".to_vec();
        let entry = TreeEntry::try_from(data.clone()).unwrap();
        assert_eq!(entry.mode, 100644);
        assert_eq!(entry.name, "file.txt");
    }
}
