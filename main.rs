#![allow(non_camel_case_types)]

use std::env;
use std::thread;
use std::time;
use std::process;
use std::io;
use std::io::Write;

pub mod ma_wrapper;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    if args.len() < 2 {
        println!("Usage: {} <audio file>", program);
        process::exit(1);
    }

    let audio_file = args[1].clone();
    println!("{audio_file}");

    let mut player = ma_wrapper::PlayerStatus { playing: 0, ended: 0, pause: 0, };

    ma_wrapper::init(&player);
    thread::spawn( move || {
        ma_wrapper::play(audio_file);
        while !ma_wrapper::is_ended() {
            sleep!(100);
        }
    });

    let mut input = String::new();
    loop {
        print!("> "); io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "play"  => player.pause = 0,
            "pause" => player.pause = 1,
            "p"     => player.pause = !player.pause,
            "q"     => break,
            _ => {},
        }
        input.clear();
    }

    ma_wrapper::uninit();
}
