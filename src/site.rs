use std::path::PathBuf;

#[derive(Clone)]
pub struct Site {
    pub content_dir: PathBuf,
    pub data_dir: PathBuf,
    pub docs_dir: PathBuf,
    pub scripts_dir: PathBuf,
}

impl Site {
    pub fn new() -> Site {
        Site {
            content_dir: PathBuf::from("content"),
            data_dir: PathBuf::from("content/_data"),
            docs_dir: PathBuf::from("docs"),
            scripts_dir: PathBuf::from("content/_scripts"),
        }
    }
}
