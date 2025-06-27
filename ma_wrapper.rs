use std::ffi::*;

#[repr(C)]
pub struct PlayerStatus {
    pub playing: c_int,
    pub pause:   c_int,
    pub ended:   c_int,
}
extern "C" {
    pub fn maw_init() -> c_int;
    pub fn maw_play(file: *const c_char) -> c_int;
    pub fn maw_is_ended() -> bool;
    pub fn maw_uninit();
    pub fn maw_get_player_status() -> *mut PlayerStatus;
}
