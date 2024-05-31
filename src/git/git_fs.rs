use crate::git::objects::blob::BlobObject;
use crate::git::objects::tree::TreeObject;
use anyhow::Result;
use flate2::read::ZlibDecoder;
use std::io::Read;

pub fn get_raw_object(path: &str) -> Result<Vec<u8>> {
    let path = path.trim();

    let (prefix, data) = path.split_at(2);

    let object_path = format!(".git/objects/{}/{}", prefix, data);

    let data = std::fs::read(object_path)?;

    // decompress the data
    let mut z = ZlibDecoder::new(&data[..]);
    let mut data = Vec::new();
    z.read_to_end(&mut data).unwrap();

    Ok(data)
}

pub fn get_blob_object(path: &str) -> Result<BlobObject> {
    let data = get_raw_object(path)?;

    BlobObject::try_from(data)
}

pub fn get_tree_object(path: &str) -> Result<TreeObject> {
    let data = get_raw_object(path)?;

    TreeObject::try_from(data)
}
