use std::collections::VecDeque;

pub fn add(history: &mut VecDeque<usize>, file_idx: usize) {
    history.push_back(file_idx)
}

pub fn get_and_pop(history: &mut VecDeque<usize>, file_idx: &mut usize) -> bool {
    match history.pop_back() {
        Some(index) => { *file_idx = index; true },
        None        => false,
    }
}
