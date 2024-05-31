use crate::git::git_fs::{get_blob_object, get_raw_object};
use crate::git::objects::header::ObjectType;
use crate::git::refs::Ref;
use anyhow::Result;
use serde::__private::from_utf8_lossy;
use std::io::BufRead;

pub async fn get_head() -> Result<String> {
    let head = tokio::fs::read_to_string(".git/HEAD").await?;
    let head = head.trim();
    let head = head.split(' ').collect::<Vec<&str>>()[1];
    Ok(head.to_string())
}

pub async fn update_head(hash: &str) -> Result<()> {
    let head = get_head().await?;

    let mut head = Ref::new(&head);

    head.update(hash).await?;

    Ok(())
}

pub async fn resolve_head() -> Result<String> {
    let head = get_head().await?;

    let ref_ = Ref::new(&Ref::parse_ref_content(&head));

    ref_.resolve().await
}

pub async fn resolve_head_to_tree() -> Result<String> {
    let head = resolve_head().await?;

    let _head = get_raw_object(&head)?;

    unimplemented!()
}

pub async fn update_current_files_to_current_head() -> Result<()> {
    // get hash of commit / tree
    let head = resolve_head().await?;

    println!("{:?}", head);

    // check if it's a commit or a tree
    let blob = get_blob_object(&head)?;

    let tree_hash = if blob.header.object_type == ObjectType::Commit {
        // get tree hash
        let body = blob.data.split(|c| *c == b'\n').collect::<Vec<&[u8]>>()[0];

        if !body.starts_with(b"tree") {
            return Err(anyhow::anyhow!("Invalid commit object"));
        }

        let tree = body.split(|c| *c == b' ').collect::<Vec<&[u8]>>()[1];

        from_utf8_lossy(tree).to_string()
    } else {
        head
    };

    let raw_object = get_raw_object(&tree_hash)?;

    let tree_obj = crate::git::objects::tree::TreeObject::try_from(raw_object)?;

    let files = tree_obj.get_files_recursive(".").await?;

    println!("{:?}", files);

    // for file in files {
    //     let file = file?;
    //     let file = file.0;
    //     let file = file.split_at(2);
    //     let file = format!(".git/objects/{}/{}", file.0, file.1);
    //     let file = std::fs::read(file)?;
    //     let blob_obj = crate::git::objects::blob::BlobObject::try_from(file)?;
    //     let file = blob_obj.get_file().await?;
    //     let path = std::path::Path::new(&file.0);
    //     let data = file.1;
    //     tokio::fs::write(path, data).await?;
    // }

    Ok(())
}
