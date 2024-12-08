use std::path::Path;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FileType {
    Rust,
    Text,
}

impl FileType {
    pub fn from_filename(filename: &str) -> Self {
        let ext = Path::new(filename).extension();
        match ext {
            Some(osstr) => osstr.to_str().map_or(Self::Text, |s| match s {
                "rs" => Self::Rust,
                _ => Self::Text,
            }),
            None => Self::Text,
        }
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::Text
    }
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rust => write!(f, "rust"),
            Self::Text => write!(f, "text"),
        }
    }
}
