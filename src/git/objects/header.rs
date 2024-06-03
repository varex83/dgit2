use crate::git::traits::ToBytes;
use anyhow::{Error, Result};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct ObjectHeader {
    pub object_type: ObjectType,
    pub size: usize,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Blob => write!(f, "blob"),
            ObjectType::Tree => write!(f, "tree"),
            ObjectType::Commit => write!(f, "commit"),
        }
    }
}

impl ObjectType {
    pub fn len(&self) -> usize {
        self.to_string().len()
    }

    pub fn is_empty(&self) -> bool {
        self.to_string().is_empty()
    }
}

impl ObjectHeader {
    pub fn header_size(&self) -> usize {
        format!("{} {}\0", self.object_type, self.size)
            .as_bytes()
            .len()
    }
}

impl ToBytes for ObjectHeader {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(format!("{} {}\0", self.object_type, self.size).as_bytes());
        result
    }
}

impl TryFrom<Vec<u8>> for ObjectHeader {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self> {
        let object_type = {
            if &value[..ObjectType::Blob.len()] == ObjectType::Blob.to_string().as_bytes() {
                ObjectType::Blob
            } else if &value[..ObjectType::Tree.len()] == ObjectType::Tree.to_string().as_bytes() {
                ObjectType::Tree
            } else if &value[..ObjectType::Commit.len()]
                == ObjectType::Commit.to_string().as_bytes()
            {
                ObjectType::Commit
            } else {
                anyhow::bail!(
                    "Invalid object header: {}",
                    std::str::from_utf8(&value).unwrap()
                );
            }
        };

        let mut size = 0;

        for item in value.iter().skip(object_type.len() + 1) {
            if *item == 0 {
                break;
            } else if (*item as char).is_ascii_digit() {
                size *= 10;
                size += <u8 as Into<usize>>::into(*item - b'0');
            } else {
                anyhow::bail!(
                    "Invalid object header: {}",
                    std::str::from_utf8(&value).unwrap()
                );
            }
        }

        Ok(ObjectHeader { object_type, size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_header_to_bytes() {
        let header = ObjectHeader {
            object_type: ObjectType::Blob,
            size: 10,
        };

        assert_eq!(header.to_bytes(), b"blob 10\0");
    }

    #[test]
    fn test_object_header_try_from() {
        let header = ObjectHeader {
            object_type: ObjectType::Blob,
            size: 10,
        };

        let bytes = header.to_bytes();

        assert_eq!(ObjectHeader::try_from(bytes).unwrap(), header);
    }
}
