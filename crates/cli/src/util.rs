use crate::error::Result;
use ignore::WalkBuilder;

/// Consolidate the contents of a directory into a single String.
pub(crate) fn generate_repo_prompt(base_path: std::path::PathBuf) -> Result<String> {
    let mut prompt = String::new();

    let walker = WalkBuilder::new(&base_path)
        .add_custom_ignore_filename(".gptignore")
        .build();

    for result in walker {
        let entry = result?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let relative_path = if path == base_path {
            // still include the base path in the prompt
            base_path.file_name().unwrap().to_str().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "path is not valid UTF-8")
            })?
        } else {
            path.strip_prefix(&base_path)?.to_str().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "path is not valid UTF-8")
            })?
        };

        let contents = std::fs::read_to_string(path)?;

        prompt.push_str("----\n");
        prompt.push_str(format!("./{}\n", relative_path).as_str());
        prompt.push_str(&contents);
    }

    Ok(prompt)
}
