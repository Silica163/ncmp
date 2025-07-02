#![allow(non_camel_case_types)]

use std::env;
use std::thread;
use std::time;
use std::process;
use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::channel;

pub mod ma_wrapper;
pub mod player;
pub mod playlist;
pub mod filelist;


use player::*;
use filelist::*;

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

fn try_exit(){
    ma_wrapper::uninit();
    println!();
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    if args.len() < 2 {
        println!("Usage: {} <audio file>", program);
        process::exit(1);
    }

    // create a list of files
    let mut audio_files: Vec<FileInfo> = vec![];
    for i in 1..args.len() {
        audio_files.push(FileInfo::new(args[i].clone()));
    }
//    println!("{audio_files:?}");

    let mut pl = playlist::shuffle(&audio_files);
//    println!("{pl:?}");

    let mut player_status = ma_wrapper::PlayerStatus { playing: 0, ended: 0, pause: 0, };
    ma_wrapper::init(&player_status);

    let command_avaliable = Arc::new(Mutex::new(false));
    let (cmd_tx, cmd_rx) = channel::<PlayerCommand>();
    let (quit_tx, quit_rx) = channel::<bool>();
    {
        let thread_command_avaliable = Arc::clone(&command_avaliable);
        let mut quit = false;
        thread::spawn(move || {
            let mut input = String::new();
            while !quit {
                print!("> "); io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let cmd = parse_command(input.trim().to_string());
                input.clear();
                cmd_tx.send(cmd).unwrap();
                *thread_command_avaliable.lock().unwrap() = true;
                quit = quit_rx.recv().unwrap();
            }

            try_exit();
        });
    }

    let mut song_idx:usize = 0;
    while playlist::next(&mut pl, &mut song_idx) {
        let song = audio_files[pl[song_idx].clone().file_idx].clone();
        println!("Playing: {}", song.name);
        ma_wrapper::play(song.path);
        while !ma_wrapper::is_ended() {
            if *command_avaliable.lock().unwrap() {
                let mut quit = false;
                *command_avaliable.lock().unwrap() = false;
                execute_command(cmd_rx.recv().unwrap(), &mut player_status, &pl, &audio_files, &mut quit);
                quit_tx.send(quit).unwrap();
            }
            sleep!(100);
        }
        pl[song_idx].played = true;
    }
    try_exit();
}
