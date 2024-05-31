use anyhow::Result;
use sha1::{Digest, Sha1};
use std::io::Write;

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Hash {
    fn hash(&self) -> String;
}

impl<T> Hash for T
where
    T: ToBytes,
{
    fn hash(&self) -> String {
        let mut hasher = Sha1::new();
        hasher.update(&self.to_bytes());
        let hash = hasher.finalize();

        hex::encode(hash)
    }
}

pub trait ObjectSave {
    fn save_object(&self) -> Result<String>;
}

impl<T> ObjectSave for T
where
    T: ToBytes + Hash,
{
    fn save_object(&self) -> Result<String> {
        let uncompressed_data = self.to_bytes();
        let hash = self.hash();

        let (prefix, hash_remainder) = hash.split_at(2);
        let object_path = format!(".git/objects/{}/{}", prefix, hash_remainder);

        // create parent directories if they don't exist
        std::fs::create_dir_all(format!(".git/objects/{}", prefix))?;

        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        z.write_all(&uncompressed_data)?;
        let compressed_data = z.finish()?;

        std::fs::write(object_path, compressed_data)?;

        Ok(hash)
    }
}
