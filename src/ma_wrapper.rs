use std::ffi::*;

#[repr(C)]
#[derive(Clone,Copy,Debug)]
pub struct PlayerStatus {
    pub playing: c_int,
    pub ended:   c_int,
    pub pause:   c_int,
}

pub enum WrapperResult {
    Success,
    Error { code: c_int },
}

pub fn init(player: *const PlayerStatus) -> Option<WrapperResult> {
    let r;
    unsafe {
        r = maw_init(player);
    }
    if r == 0 { return Some(WrapperResult::Success); }
    Some(WrapperResult::Error { code: r })
}

pub fn uninit() {
    unsafe {
        maw_uninit();
    }
}

pub fn is_ended() -> bool {
    unsafe {
        maw_is_ended()
    }
}

pub fn play(file: String) ->  Option<WrapperResult> {
    let r;
    unsafe {
        r = maw_play(CString::new(file).unwrap().as_ptr());
    }
    if r == 0 { return Some(WrapperResult::Success); }
    Some(WrapperResult::Error { code: r })
}

pub fn get_player_status() -> *mut PlayerStatus {
    unsafe {
        maw_get_player_status()
    }
}

pub fn get_length_in_secs() -> i32 {
    unsafe {
        maw_get_length_in_secs()
    }
}

pub fn get_cursor_in_secs() -> i32 {
    unsafe {
        maw_get_cursor_in_secs()
    }
}

pub fn seek_to_sec(sec: i32) -> Option<WrapperResult> {
    let r;
    unsafe {
        r = maw_seek_to_sec(sec);
    }
    if r == 0 { return Some(WrapperResult::Success); }
    Some(WrapperResult::Error { code: r })
}

extern "C" {
    fn maw_init(player: *const PlayerStatus) -> c_int;
    fn maw_play(file: *const c_char) -> c_int;
    fn maw_is_ended() -> bool;
    fn maw_uninit();
    fn maw_get_player_status() -> *mut PlayerStatus;
    fn maw_get_length_in_secs() -> c_int;
    fn maw_get_cursor_in_secs() -> c_int;
    fn maw_seek_to_sec(sec: c_int) -> c_int;
}
