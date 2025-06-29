#![allow(non_camel_case_types)]

use std::env;
use std::thread;
use std::time;
use std::process;
use std::io;
use std::io::Write;

pub mod ma_wrapper;
pub mod player;

use player::*;

#[macro_export]
macro_rules! c {
    ($l:expr) => {
        concat!($l, "\0").as_ptr() as *const c_char
    }
}

macro_rules! sleep {
    ($ms:expr) => {
        thread::sleep(time::Duration::from_millis($ms));
    };
}

macro_rules! time_rand {
    ($max:expr) => {
        (time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_micros() as usize) % $max
    }
}

fn try_exit(){
    ma_wrapper::uninit();
    println!();
    process::exit(1);
}

#[derive(Debug, Clone)]
struct PlaylistItem {
    file: String,
    file_idx: usize,
    played: bool,
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    if args.len() < 2 {
        println!("Usage: {} <audio file>", program);
        process::exit(1);
    }

    let mut audio_files: Vec<String> = vec![];
    let mut playlist: Vec<PlaylistItem> = vec![];
    for i in 1..args.len() {
        audio_files.push(args[i].clone());
        playlist.push(PlaylistItem::new_empty());
    }
    println!("{audio_files:?}");

    for i in 0..audio_files.len() {
        let idx = {
            let mut idx = time_rand!(audio_files.len());
            while playlist[idx].file != "" {
                idx = time_rand!(audio_files.len());
            }
            idx
        };
        playlist[idx] = PlaylistItem::new(i, audio_files[i].clone());
    }

    let mut player_status = ma_wrapper::PlayerStatus { playing: 0, ended: 0, pause: 0, };
    ma_wrapper::init(&player_status);
    thread::spawn( move || {
        let mut playlist_ended = false;
        let mut play_all = true;
        let mut song_idx = 0;
        while !playlist_ended {
            let song = playlist[song_idx].clone();
            if !song.played {
                let file = song.file.clone();
                println!("Playing: {}", file.rsplitn(2,"/").collect::<Vec<&str>>()[0]);
                ma_wrapper::play(file);
                while !ma_wrapper::is_ended() {
                    sleep!(100);
                }
                playlist[song_idx].played = true;
            }

            song_idx += 1;
            if song_idx >= playlist.len() {
                song_idx = 0;
                play_all = true;
            }
            play_all &= playlist[song_idx].played;
            playlist_ended = play_all;
        }
        try_exit();
    });

    let mut quit = false;
    let mut input = String::new();
    while !quit {
        print!("> "); io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = parse_command(input.trim().to_string());
        execute_command(cmd, &mut player_status, &mut quit);
        input.clear();
    }

    try_exit();
}
