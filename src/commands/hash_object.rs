use crate::git::objects::blob::BlobObject;
use crate::git::traits::Hash;
use anyhow::Result;
use std::path::Path;

pub fn hash_object(file_name: &str) -> Result<()> {
    let hash = BlobObject::write_blob_object(Path::new(file_name))?.hash();

    println!("{}", hash);

    Ok(())
}
