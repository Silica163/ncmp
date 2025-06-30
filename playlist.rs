use std::time;
macro_rules! time_rand {
    ($max:expr) => {
        (time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_micros() as usize) % $max
    }
}

#[derive(Debug, Clone)]
pub struct PlaylistItem {
    pub file: String,
    pub file_idx: usize,
    pub played: bool,
}

impl PlaylistItem {
    pub fn new_empty() -> Self {
        Self {
            file: "".to_string(),
            file_idx: 0,
            played: false,
        }
    }
    pub fn new(idx: usize, file: String) -> Self {
        Self {
            file: file,
            file_idx: idx,
            played: false,
        }
    }
}

pub fn playlist_shuffle(files: &Vec<String>) -> Vec<PlaylistItem> {
    let mut playlist: Vec<PlaylistItem> = vec![PlaylistItem::new_empty(); files.len()];
    let mut avaliable_slot: Vec<usize> = (0..files.len()).collect();
    for i in 0..files.len() {
        let idx = {
            let idx = time_rand!(avaliable_slot.len());
            avaliable_slot.remove(idx)
        };
        playlist[idx] = PlaylistItem::new(i, files[i].clone());
    }
    playlist
}
