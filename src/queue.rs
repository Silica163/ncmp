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

// return true on success
pub fn dequeue_at(
    queue: &mut VecDeque<usize>, queue_idx: usize
) -> bool {
    if queue.len() == 0 || queue_idx >= queue.len(){ return false }
    if queue_idx == 0 {
        queue.pop_front();
    } else {
        queue.remove(queue_idx);
    }
    true
}

pub fn show(queue: &VecDeque<usize>, files: &BTreeMap<usize, filelist::FileInfo>) {
    println!("=========== queue ============");
    for (index, file_idx) in queue.iter().enumerate() {
        match files.get(&file_idx) {
            Some(file) => println!("{index:03}: {}", file.name),
            None => { println!("file id {index:03} is not exists in file list.")},
        }
    }
    println!("==============================");
}
