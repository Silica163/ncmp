#![allow(non_camel_case_types)]

use std::env;
use std::thread;
use std::time;
use std::process;
use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

pub mod ma_wrapper;
pub mod player;
pub mod playlist;


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
    let mut audio_files: Vec<String> = vec![];
    for i in 1..args.len() {
        audio_files.push(args[i].clone());
    }
    println!("{audio_files:?}");

    let pl = Arc::new(Mutex::new(playlist::shuffle(&audio_files)));
    println!("{pl:?}");

    let mut player_status = ma_wrapper::PlayerStatus { playing: 0, ended: 0, pause: 0, };
    ma_wrapper::init(&player_status);
    {
        let pl2 = Arc::clone(&pl);
        thread::spawn(move || {
            let mut song_idx:usize = 0;
            while playlist::next(&mut pl2.lock().unwrap(), &mut song_idx) {
                println!("Playing: {}", pl2.lock().unwrap()[song_idx].name.to_string());
                ma_wrapper::play(pl2.lock().unwrap()[song_idx].file.to_string());
                while !ma_wrapper::is_ended() {
                    sleep!(100);
                }
                pl2.lock().unwrap()[song_idx].played = true;
            }
            try_exit();
        });
    }

    let mut quit = false;
    let mut input = String::new();
    while !quit {
        print!("> "); io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = parse_command(input.trim().to_string());
        execute_command(cmd, &mut player_status, &pl.lock().unwrap(), &mut quit);
        input.clear();
    }

    try_exit();
}
