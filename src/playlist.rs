use std::time;
use filelist;

macro_rules! time_rand {
    ($max:expr) => {
        (time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_micros() as usize) % $max
    }
}

#[derive(Debug, Clone)]
pub struct PlaylistItem {
    pub file_idx: usize,
    pub played: bool,
}

impl PlaylistItem {
    pub fn new_empty() -> Self {
        Self {
            file_idx: 0,
            played: false,
        }
    }
    pub fn new(idx: usize) -> Self {
        Self {
            file_idx: idx,
            played: false,
        }
    }
}

pub fn shuffle(files: &Vec<filelist::FileInfo>) -> Vec<PlaylistItem> {
    let mut playlist: Vec<PlaylistItem> = vec![PlaylistItem::new_empty(); files.len()];
    let mut avaliable_slot: Vec<usize> = (0..files.len()).collect();
    for i in 0..files.len() {
        let idx = {
            let idx = time_rand!(avaliable_slot.len());
            avaliable_slot.remove(idx)
        };
        playlist[idx] = PlaylistItem::new(i);
    }
    playlist
}

pub fn next(playlist: &mut Vec<PlaylistItem>, current_song: &mut usize) -> bool {
    let mut next_song = *current_song;
    for _ in 0..(playlist.len()+1) {
        if !playlist[next_song].played {
            *current_song = next_song;
            return true
        }
        next_song = (next_song + 1) % playlist.len();
    }
    return false
}

pub fn is_ended(playlist: &Vec<PlaylistItem>) -> bool {
    for item in playlist {
        if item.played { continue }
        return false
    }
    return true
}

pub fn show(playlist: &Vec<PlaylistItem>, files: &Vec<filelist::FileInfo>){
    println!("========== playlist ==========");
    for (index, item) in playlist.iter().enumerate() {
        println!("{index:03}: {}", files[item.file_idx].name);
    }
    println!("==============================");
}
