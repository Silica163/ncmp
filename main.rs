#![allow(non_camel_case_types)]

use std::env;
use std::thread;
use std::time;
use std::process;
use std::ffi::*;

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

    let mut player: ma_wrapper::PlayerStatus = ma_wrapper::PlayerStatus {
        playing: 0,
        pause: 0,
        ended: 0,
    };
    ma_wrapper::init(&player);
    ma_wrapper::play(audio_file);
    while !ma_wrapper::is_ended() {
        sleep!(100);
    }
    ma_wrapper::uninit();
/*
    let mut input = String::new();
    loop {
        print!("> "); io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        println!("=> {}", input.trim());
        if input.trim() == "exit" { return Ok(()) }
        input.clear();
    }
*/
}
