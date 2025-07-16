use std::fs;
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub length: i32,
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
        match path.as_str().rsplit_once('.') {
            Some((_, ext)) => match ext {
                "mp3"|"flac"|"ogg"|"wav" => files.push(path),
                other => println!("Unsupported extension {other} : `{}`", path),
            },
            None => println!("Unknown extension: {path}"),
        }
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

pub fn remove(files: &mut BTreeMap<usize, FileInfo>, id: usize) {
    match files.remove(&id) {
        Some(file)  => {
            println!("file {} removed.", file.name);
        },
        None        => {
            println!("file id {id} is not exist.");
        },
    }
}

fn path_match(file: FileInfo, pattern: String) -> bool {
    file.path.contains(&pattern)
}

pub fn remove_by_pattern(files: &mut BTreeMap<usize, FileInfo>, pattern: String) {
    let mut to_be_remove: Vec<usize> = vec![];
    for (file_idx, file) in files.iter() {
        if path_match(file.clone(), pattern.clone()) {
            to_be_remove.push(*file_idx);
        }
    }
    for file_idx in to_be_remove {
        match files.remove(&file_idx) {
            Some(_) => {},
            None    => unreachable!("file_idx should exists in filelist"),
        }
    }
}
