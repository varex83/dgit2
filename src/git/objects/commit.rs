use crate::git::objects::header::{ObjectHeader, ObjectType};
use crate::git::traits::ToBytes;

#[derive(Debug, Clone)]
pub struct CommitObject {
    pub header: ObjectHeader,
    pub content: CommitContent,
}

#[derive(Debug, Clone)]
pub struct CommitContent {
    pub tree_sha: String,
    pub parent_sha: Vec<String>,
    pub author: CommitAuthor,
    pub committer: CommitAuthor,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub timestamp: i64,
    pub timezone: String,
}

impl CommitObject {
    pub fn new(content: CommitContent) -> Self {
        CommitObject {
            header: ObjectHeader {
                object_type: ObjectType::Commit,
                size: content.to_bytes().len(),
            },
            content,
        }
    }
}

impl ToBytes for CommitObject {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(self.header.to_bytes().as_slice());
        result.extend_from_slice(self.content.to_bytes().as_slice());
        result
    }
}

impl ToBytes for CommitAuthor {
    fn to_bytes(&self) -> Vec<u8> {
        format!(
            "{} <{}> {} {}",
            self.name, self.email, self.timestamp, self.timezone
        )
        .as_bytes()
        .to_vec()
    }
}

impl ToBytes for CommitContent {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(format!("tree {}\n", self.tree_sha).as_bytes());

        for parent in &self.parent_sha {
            result.extend_from_slice(format!("parent {}\n", parent).as_bytes());
        }

        result.extend_from_slice("author ".as_bytes());
        result.extend_from_slice(self.author.to_bytes().as_slice());
        result.extend_from_slice("\ncommitter ".to_string().as_bytes());
        result.extend_from_slice(self.committer.to_bytes().as_slice());
        result.extend_from_slice(format!("\n\n{}\n", self.message).as_bytes());

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_author_to_bytes() {
        let author = CommitAuthor {
            name: String::from("John Doe"),
            email: String::from("john@example.com"),
            timestamp: 1622519072,
            timezone: String::from("+0000"),
        };

        let bytes = author.to_bytes();
        let expected = b"John Doe <john@example.com> 1622519072 +0000".to_vec();

        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_commit_content_to_bytes() {
        let author = CommitAuthor {
            name: String::from("John Doe"),
            email: String::from("john@example.com"),
            timestamp: 1622519072,
            timezone: String::from("+0000"),
        };

        let content = CommitContent {
            tree_sha: String::from("abc123"),
            parent_sha: vec![String::from("def456"), String::from("ghi789")],
            author: author.clone(),
            committer: author.clone(),
            message: String::from("Initial commit"),
        };

        let bytes = content.to_bytes();
        let expected = b"tree abc123\nparent def456\nparent ghi789\nauthor John Doe <john@example.com> 1622519072 +0000\ncommitter John Doe <john@example.com> 1622519072 +0000\nInitial commit\n".to_vec();

        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_commit_object_to_bytes() {
        let author = CommitAuthor {
            name: String::from("John Doe"),
            email: String::from("john@example.com"),
            timestamp: 1622519072,
            timezone: String::from("+0000"),
        };

        let content = CommitContent {
            tree_sha: String::from("abc123"),
            parent_sha: vec![String::from("def456"), String::from("ghi789")],
            author: author.clone(),
            committer: author.clone(),
            message: String::from("Initial commit"),
        };

        let commit_object = CommitObject::new(content.clone());

        let mut expected = Vec::new();
        expected.extend_from_slice(
            ObjectHeader {
                object_type: ObjectType::Commit,
                size: content.to_bytes().len(),
            }
            .to_bytes()
            .as_slice(),
        );
        expected.extend_from_slice(content.to_bytes().as_slice());

        assert_eq!(commit_object.to_bytes(), expected);
    }
}
