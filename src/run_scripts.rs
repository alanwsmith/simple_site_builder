use anyhow::Result;
use anyhow::anyhow;
use itertools::Itertools;
use permissions::is_executable;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

pub fn run_scripts(dir: &PathBuf) -> Result<()> {
    if !dir.is_dir() {
        return Err(anyhow!("Not a directory at: {}", dir.display()));
    }
    let walker = WalkDir::new(dir).into_iter();
    let files: Vec<_> = walker
        .filter_map(|entry| match entry {
            Ok(e) => Some(e.path().to_path_buf()),
            Err(_) => None,
        })
        .filter(|pb| pb.is_file())
        .filter(|pb| {
            pb.components()
                .find(|part| part.as_os_str().to_string_lossy().starts_with("."))
                .is_none()
        })
        .sorted()
        .collect();
    for file in files {
        if is_executable(&file)? {
            let name = file.file_name().ok_or(anyhow!("Cound not get file name"))?;
            let parent = file.parent().ok_or(anyhow!("Could not get parent"))?;
            let canon_parent = canonicalize(parent)?;
            println!("Running: {}", file.display());
            Command::new(format!("./{}", name.display()))
                .current_dir(canon_parent)
                .spawn()?;
        }
    }
    Ok(())
}
