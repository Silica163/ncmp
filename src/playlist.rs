use std::time;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use filelist;

macro_rules! time_rand {
    ($max:expr) => {
        (time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_micros() as usize) % $max
    }
}

pub fn re_shuffle(files: &mut BTreeMap<usize, filelist::FileInfo>, playlist: &mut VecDeque<usize>) {
    for _ in 0..files.len() {
        playlist.push_back(0)
    }

    let mut avaliable_slot: Vec<usize> = (0..files.len()).collect();
    for i in files.clone().keys() {
        let idx = {
            let idx = time_rand!(avaliable_slot.len());
            avaliable_slot.remove(idx)
        };
        playlist[idx] = *i;
    }
}

pub fn shuffle(files: &mut BTreeMap<usize, filelist::FileInfo>) -> VecDeque<usize> {
    let mut playlist: VecDeque<usize> = VecDeque::new();
    re_shuffle(files, &mut playlist);
    playlist
}

// get next song
// return false when playlist is ended
pub fn next(playlist: &mut VecDeque<usize>, file_idx: &mut usize) -> bool {
    match playlist.pop_front() {
        Some(idx)  => {*file_idx = idx; true },
        None        => false,
    }
}

pub fn update(playlist: &mut VecDeque<usize>, files: &BTreeMap<usize, filelist::FileInfo>){
    for (playlist_index, file_idx) in playlist.clone().iter().enumerate() {
        match files.get(&(file_idx)) {
            Some(_) => {},
            None => { playlist.remove(playlist_index); break; },
        }
    }
}
