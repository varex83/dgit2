use crate::git::git_fs::get_tree_object;
use anyhow::Result;

pub fn ls_tree(path: &str, is_names_only: bool) -> Result<()> {
    let tree_object = get_tree_object(path)?;

    for entry in tree_object.entries {
        if is_names_only {
            println!("{}", entry.name);
        } else {
            println!("{:06}\t{:20}\t{:30}", entry.mode, entry.name, entry.hash);
        }
    }

    Ok(())
}
