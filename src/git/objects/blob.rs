use crate::git::objects::header::{ObjectHeader, ObjectType};
use crate::git::traits::{ObjectSave, ToBytes};
use anyhow::Result;
use std::convert::TryFrom;
use std::path::Path;

// Writing Git in Rust

#[derive(Debug)]
pub struct BlobObject {
    pub header: ObjectHeader,
    pub data: Vec<u8>,
}

impl BlobObject {
    pub fn new(data: Vec<u8>) -> Self {
        BlobObject {
            header: ObjectHeader {
                object_type: ObjectType::Blob,
                size: data.len(),
            },
            data,
        }
    }

    pub fn write_blob_object(path: &Path) -> Result<BlobObject> {
        let blob_data = std::fs::read_to_string(path)?;

        let blob_object = BlobObject::new(blob_data.as_bytes().to_vec());

        let _ = blob_object.save_object()?;

        Ok(blob_object)
    }
}

impl ToBytes for BlobObject {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(format!("blob {}\0", self.header.size).as_bytes());
        result.extend_from_slice(&self.data);
        result
    }
}

impl TryFrom<Vec<u8>> for BlobObject {
    type Error = anyhow::Error;

    fn try_from(data: Vec<u8>) -> Result<Self> {
        // blob <size>\0<data>

        let header = ObjectHeader::try_from(data.clone())?;

        let header_size = header.header_size();

        Ok(BlobObject {
            header,
            data: data[header_size..].to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blob_object_from() {
        let data = b"blob 5\0hello".to_vec();
        let blob = BlobObject::try_from(data.clone()).unwrap();
        assert_eq!(blob.header.size, 5);
        assert_eq!(blob.data, b"hello");
    }

    #[test]
    fn test_blob_object_from_invalid() {
        let data = b"blob 5hello".to_vec();
        let result = BlobObject::try_from(data.clone());
        assert!(result.is_err());
    }
}
