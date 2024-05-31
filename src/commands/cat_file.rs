use crate::git::git_fs;
use anyhow::bail;
use anyhow::Result;

pub fn cat_file(path: &str) -> Result<()> {
    // verify that the length of the path is sha1 length
    if path.len() != 40 {
        bail!("Invalid object path")
    }

    let object = match git_fs::get_blob_object(path) {
        Ok(object) => object,
        Err(e) => {
            bail!("Failed to get object: {}", e)
        }
    };

    print!("{}", std::str::from_utf8(&object.data)?);

    Ok(())
}
