use std::path::Path;

pub fn extract_name_file(file_name: &Path) -> Option<&str> {
    let os_name = file_name.file_stem().and_then(|s| s.to_str());
    os_name
}
