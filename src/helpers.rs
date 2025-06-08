use anyhow::{Result, anyhow};
use itertools::Itertools;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[allow(unused)]
pub fn copy_file_list_from_to(
    files: &Vec<PathBuf>,
    from: &PathBuf,
    to: &PathBuf,
    overwrite: bool,
) -> Result<()> {
    for file in files.iter() {
        let mut do_copy = true;
        let out_path = to.join(file);
        if out_path.exists() && !overwrite {
            do_copy = false;
        }
        if do_copy {
            let in_path = from.join(file);
            let out_parent = out_path.parent().unwrap();
            fs::create_dir_all(&out_parent)?;
            let data = std::fs::read(in_path)?;
            std::fs::write(out_path, &data)?;
        }
    }
    Ok(())
}

pub fn empty_dir(dir: &PathBuf) -> Result<()> {
    if let Ok(exists) = dir.try_exists() {
        if exists {
            for entry in dir.read_dir()? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(path)?;
                } else {
                    fs::remove_file(path)?;
                }
            }
        }
    }
    Ok(())
}

pub fn get_files_in_dir(
    dir: &PathBuf,
    with: Option<Vec<&str>>,
    without: Option<Vec<&str>>,
) -> Result<Vec<PathBuf>> {
    Ok(fs::read_dir(dir)?
        .into_iter()
        .filter_map(|path| path.ok())
        .map(|path| path.path().to_path_buf())
        .filter(|path| path.is_file())
        .filter_map(|path| match path.strip_prefix(dir) {
            Ok(p) => Some(p.to_path_buf()),
            Err(_) => None,
        })
        .filter(|path| !path.display().to_string().starts_with("."))
        .filter(|path| {
            if let Some(with) = &with {
                if let Some(ext) = path.extension() {
                    with.contains(&ext.to_str().unwrap())
                } else {
                    false
                }
            } else {
                true
            }
        })
        .filter(|path| {
            if let Some(without) = &without {
                if let Some(ext) = path.extension() {
                    !without.contains(&ext.to_str().unwrap())
                } else {
                    false
                }
            } else {
                true
            }
        })
        .collect())
}

pub fn get_files_in_tree(
    dir: &PathBuf,
    with: Option<Vec<&str>>,
    without: Option<Vec<&str>>,
) -> Result<Vec<PathBuf>> {
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
        .filter_map(|path| match path.strip_prefix(dir) {
            Ok(p) => Some(p.to_path_buf()),
            Err(_) => None,
        })
        .filter(|pb| {
            pb.components()
                .find(|part| part.as_os_str().to_string_lossy().starts_with("."))
                .is_none()
        })
        .filter(|pb| {
            if let Some(with) = &with {
                if let Some(ext) = pb.extension() {
                    return with
                        .iter()
                        .map(|e| e.to_lowercase())
                        .contains(&ext.to_str().unwrap().to_lowercase());
                }
            }
            true
        })
        .filter(|pb| {
            if let Some(without) = &without {
                if let Some(ext) = pb.extension() {
                    return !without
                        .iter()
                        .map(|e| e.to_lowercase())
                        .contains(&ext.to_str().unwrap().to_lowercase());
                }
            }
            true
        })
        .sorted()
        .collect();
    Ok(files)
}
