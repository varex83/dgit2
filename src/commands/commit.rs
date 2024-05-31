use crate::git::head::{resolve_head, update_head};
use crate::git::objects::commit::{CommitAuthor, CommitContent, CommitObject};
use crate::git::objects::tree::TreeObject;
use crate::git::traits::{Hash, ObjectSave};
use crate::git::utils::detect_git_dir;
use std::path::Path;

pub async fn commit(message: Option<String>) -> anyhow::Result<()> {
    let message = message.unwrap_or_else(|| "<blank>".to_string());

    let head = resolve_head().await?;
    let parent = if head.is_empty() { vec![] } else { vec![head] };

    let tree_obj = TreeObject::write_tree_object(Path::new(&detect_git_dir()?))?;
    let tree_hash = tree_obj.hash();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    let commit_object = CommitObject::new(CommitContent {
        tree_sha: tree_hash,
        parent_sha: parent,
        author: CommitAuthor {
            name: "Varex".to_string(),
            email: "example@exampe.com".to_string(),
            timestamp,
            timezone: "+0000".to_string(),
        },
        committer: CommitAuthor {
            name: "Varex".to_string(),
            email: "example@exampe.com".to_string(),
            timestamp,
            timezone: "+0000".to_string(),
        },
        message,
    });

    commit_object.save_object()?;

    let hash = commit_object.hash();
    update_head(&hash).await?;

    println!("{}", hash);

    Ok(())
}
