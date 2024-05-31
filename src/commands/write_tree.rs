use crate::git::objects::tree::TreeObject;
use crate::git::traits::Hash;
use anyhow::Result;
use std::path::Path;

pub fn write_tree(path: &str) -> Result<()> {
    let tree_object = TreeObject::write_tree_object(Path::new(path))?;

    println!("{}", tree_object.hash());

    Ok(())
}
