pub fn add_changed_to_filename(path: &str) -> String {
    let path = std::path::Path::new(path);

    let dir = path.parent().unwrap_or(std::path::Path::new(""));
    let filename = path.file_name().unwrap_or_default().to_str().unwrap_or("");

    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            return dir.join(format!("{}_changed.{}", stem, ext))
                .to_str()
                .unwrap_or(path.to_str().unwrap_or(""))
                .to_string();
        }
    }

    dir.join(format!("{}_changed", filename))
        .to_str()
        .unwrap_or(path.to_str().unwrap_or(""))
        .to_string()
}