use std::fs;
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub length: i32,
    pub playlist_index: usize,
}

impl FileInfo {
    pub fn new(path: String) -> Self {
        Self {
            path: path.clone(),
            name: path.rsplitn(2, "/").collect::<Vec<&str>>()[0].to_string(),
            length: 0,
            playlist_index: 0,
        }
    }
}

pub fn scan_and_sort_path(paths: Vec<String>, files: &mut BTreeMap<usize, FileInfo>) {
    let mut files_str: Vec<String> = vec![];
    for path in paths.iter() {
        scan_path(path.to_string(), &mut files_str);
    }
    files_str.sort();
    for (id, path) in files_str.iter().enumerate() {
        files.insert(id, FileInfo::new(path.to_string()));
    }
}

pub fn scan_path(path: String, files: &mut Vec<String>) -> Option<()> {
    let path_type = fs::symlink_metadata(&path).ok()?.file_type();
    if path_type.is_symlink() { println!("{} todo: how to scan symlink?", path); return Some(()) }
    if path_type.is_dir() {
        for entry in fs::read_dir(&path).ok()? {
            scan_path(entry.ok()?.path().display().to_string(), files)?;
        }
    }
    if path_type.is_file() {
        files.push(path);
    }
    Some(())
}

pub fn show(files: &BTreeMap<usize, FileInfo>, full_path: bool) {
    println!("========== files =============");
    for (id, file) in files.iter() {
        println!("{id:03}: {}", if full_path { file.path.clone() } else { file.name.clone() });
    }
    println!("==============================");
}

pub fn remove(files: &mut BTreeMap<usize, FileInfo>, id: usize) -> Option<usize> {
    match files.remove(&id) {
        Some(file)  => {
            println!("file {} removed.", file.name);
            Some(file.playlist_index)
        },
        None        => {
            println!("file id {id} is not exist.");
            None
        },
    }
}
