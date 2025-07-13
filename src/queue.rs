use std::collections::BTreeMap;
use std::collections::VecDeque;
use filelist;

// return false when queue is empty
pub fn next(queue: &mut VecDeque<usize>, file_idx: &mut usize) -> bool {
    match queue.pop_front() {
        Some(item) => {
            *file_idx = item;
            true
        },
        None => false,
    }
}

// return true on success
pub fn enqueue_at(
    queue: &mut VecDeque<usize>, queue_idx: usize,
    file_idx: usize, files: &BTreeMap<usize, filelist::FileInfo>
) -> bool {
    if !files.contains_key(&file_idx){ return false }
    if queue_idx >= queue.len() {
        queue.push_back(file_idx);
    } else {
        queue.insert(queue_idx, file_idx);
    }
    true
}

pub fn enqueue_many(
    queue: &mut VecDeque<usize>,
    file_idxs: Vec<usize>,
    files: &BTreeMap<usize, filelist::FileInfo>
) {
    for file_idx in file_idxs {
        if files.contains_key(&file_idx) {
            queue.push_back(file_idx);
        } else {
            println!("file id {file_idx:3} does not exist in filelist.");
        }
    }
}

// return index on success
pub fn dequeue_at(
    queue: &mut VecDeque<usize>, queue_idx: usize
) -> Option<usize> {
    if queue.len() == 0 || queue_idx >= queue.len(){ return None }
    Some(if queue_idx == 0 {
        queue.pop_front().unwrap()
    } else {
        queue.remove(queue_idx).unwrap()
    })
}
