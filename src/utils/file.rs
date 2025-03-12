use std::fs;
use std::path::Path;
use std::io::{Result, ErrorKind, Error};

pub fn delete_files_except_tree_jpg() -> Result<()> {
    const TARGET: &str = "files/img";
    const PROTECTED: &str = "tree.jpg";
    let img_dir = Path::new(TARGET);
    if !img_dir.exists() || !img_dir.is_dir() {
        let msg = format!(" {} not found!", TARGET);
        return Err(Error::new(ErrorKind::NotFound, msg));
    }
    for entry in fs::read_dir(img_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.file_name().and_then(|n| n.to_str()) == Some(PROTECTED) {
            continue;
        }
        fs::remove_file(&path)?;
    }
    Ok(())
}

pub fn add_suffix_to_filename(path: &str, suffix: &str) -> String {
    let path = std::path::Path::new(path);

    let dir = path.parent().unwrap_or(std::path::Path::new(""));
    let filename = path.file_name().unwrap_or_default().to_str().unwrap_or("");

    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            return dir.join(format!("{}_{}.{}", stem, suffix, ext))
                .to_str()
                .unwrap_or(path.to_str().unwrap_or(""))
                .to_string();
        }
    }

    dir.join(format!("{}_{}", filename, suffix))
        .to_str()
        .unwrap_or(path.to_str().unwrap_or(""))
        .to_string()
}