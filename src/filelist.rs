use std::fs;
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub length: u32,
}

impl FileInfo {
    pub fn new(path: String) -> Self {
        Self {
            path: path.clone(),
            name: path.rsplitn(2, "/").collect::<Vec<&str>>()[0].to_string(),
            length: 0,
        }
    }
}

pub fn scan_path(path: String, files: &mut Vec<FileInfo>) -> Option<()> {
    let path_type = fs::symlink_metadata(&path).ok()?.file_type();
    if path_type.is_symlink() { println!("{} how to scan symlink?", path); return Some(()) }
    if path_type.is_dir() {
        for entry in fs::read_dir(&path).ok()? {
            scan_path(entry.ok()?.path().display().to_string(), files)?;
        }
    }
    if path_type.is_file() {
        files.push(FileInfo::new(path));
    }
    Some(())
}
