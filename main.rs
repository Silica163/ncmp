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
    ($ms:expr) =>{
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

    let audio_file = args[1].clone();
    println!("{audio_file}");

    let mut player_status = ma_wrapper::PlayerStatus { playing: 0, ended: 0, pause: 0, };

    ma_wrapper::init(&player_status);
    thread::spawn( move || {
        ma_wrapper::play(audio_file);
        while !ma_wrapper::is_ended() {
            sleep!(100);
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
