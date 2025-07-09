use std::collections::BTreeMap;
use std::collections::VecDeque;
use filelist;
use playlist;

#[derive(Clone, Debug)]
pub struct QueueItem {
    file_idx: usize,
    playlist_index: usize,
}

impl QueueItem {
    pub fn from_file(file: (&usize, &filelist::FileInfo)) -> Self {
        let (file_idx, file) = file;
        Self{
            file_idx: *file_idx,
            playlist_index: file.playlist_index,
        }
    }

    pub fn from_file_idx(file_idx: usize, files: &BTreeMap<usize, filelist::FileInfo>) -> Self {
        Self::from_file(files.get_key_value(&file_idx).unwrap())
    }

    pub fn from_playlist(playlist_item: &playlist::PlaylistItem, files: &BTreeMap<usize, filelist::FileInfo>) -> Self {
        Self::from_file(files.get_key_value(&playlist_item.file_idx).unwrap())
    }
}

pub fn next(queue: &mut VecDeque<QueueItem>, file_idx: &mut usize) -> bool {
    match queue.pop_front() {
        Some(item) => {
            *file_idx = item.file_idx;
            true
        },
        None => false,
    }
}

pub fn show(queue: &VecDeque<QueueItem>, files: &BTreeMap<usize, filelist::FileInfo>) {
    println!("=========== queue ============");
    for (index, item) in queue.iter().enumerate() {
        match files.get(&(item.file_idx)) {
            Some(file) => println!("{index:03}: {}", file.name),
            None => { println!("file id {index:03} is not exists in file list.")},
        }
    }
    println!("==============================");
}
