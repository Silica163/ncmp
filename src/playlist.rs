use std::time;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use filelist;

macro_rules! time_rand {
    ($max:expr) => {
        (time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_micros() as usize) % $max
    }
}

#[derive(Debug, Clone)]
pub struct PlaylistItem {
    pub file_idx: usize,
}

impl PlaylistItem {
    pub fn new_empty() -> Self {
        Self {
            file_idx: 0,
        }
    }
    pub fn new(idx: usize) -> Self {
        Self {
            file_idx: idx,
        }
    }
}

pub fn re_shuffle(files: &mut BTreeMap<usize, filelist::FileInfo>, playlist: &mut VecDeque<PlaylistItem>) {
    for _ in 0..files.len() {
        playlist.push_back(PlaylistItem::new_empty())
    }

    let mut avaliable_slot: Vec<usize> = (0..files.len()).collect();
    for i in files.clone().keys() {
        let idx = {
            let idx = time_rand!(avaliable_slot.len());
            avaliable_slot.remove(idx)
        };
        playlist[idx] = PlaylistItem::new(*i);
    }
}

pub fn shuffle(files: &mut BTreeMap<usize, filelist::FileInfo>) -> VecDeque<PlaylistItem> {
    let mut playlist: VecDeque<PlaylistItem> = VecDeque::new();
    re_shuffle(files, &mut playlist);
    playlist
}

// get next song
// return false when playlist is ended
pub fn next(playlist: &mut VecDeque<PlaylistItem>, file_idx: &mut usize) -> bool {
    match playlist.pop_front() {
        Some(song)  => {*file_idx = song.file_idx; true },
        None        => false,
    }
}

pub fn show(playlist: &VecDeque<PlaylistItem>, files: &BTreeMap<usize, filelist::FileInfo>){
    println!("========== playlist ==========");
    for (index, item) in playlist.iter().enumerate() {
        match files.get(&(item.file_idx)) {
            Some(file) => println!("{index:03}: {}", file.name),
            None => { println!("file id {index:03} is not exists in file list.")},
        }
    }
    println!("==============================");
}

pub fn update(playlist: &mut VecDeque<PlaylistItem>, files: &BTreeMap<usize, filelist::FileInfo>){
    for (index, item) in playlist.clone().iter().enumerate() {
        match files.get(&(item.file_idx)) {
            Some(_) => {},
            None => { playlist.remove(index); break; },
        }
    }
}
