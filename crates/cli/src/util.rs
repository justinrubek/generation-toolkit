use crate::error::Result;

pub(crate) struct RepoPromptGenerator {
    pub(crate) directory: std::path::PathBuf,
    pub(crate) ignore_files: Vec<String>,
}

impl RepoPromptGenerator {
    pub(crate) fn new(directory: std::path::PathBuf, ignore_files: Vec<String>) -> Self {
        Self {
            directory,
            ignore_files,
        }
    }

    pub(crate) async fn generate(&self) -> Result<String> {
        Ok("".to_string())
    }
}
