#![allow(non_camel_case_types)]

use std::env;
use std::io;
use std::thread;
use std::time;
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

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    if args.len() < 2 {
        panic!("Usage: {} <audio file>", program);
    }

    let mut audio_file = String::new();
    audio_file = args[1].clone();
    println!("{audio_file}");

    unsafe {
        ma_wrapper::maw_init();
        if 0 !=  ma_wrapper::maw_play(CString::new(audio_file)?.as_ptr()) {
            return Ok(())
        }
        while !ma_wrapper::maw_is_ended() {
            sleep!(100);
        }
        
//        if 0 != ma_wrapper::maw_play(c!("/home/silica/Music/RUS/Durnoy Vkus/Durnoy Vkus - Plastinki (Records).mp3")) {
//            return Ok(())
//        }
//        while !ma_wrapper::maw_is_ended() {}
        ma_wrapper::maw_uninit();
    }
    Ok(())

//    let mut input = String::new();
//    loop {
//        print!("> "); io::stdout().flush()?;
//        io::stdin().read_line(&mut input)?;
//        println!("=> {}", input.trim());
//        if input.trim() == "exit" { return Ok(()) }
//        input.clear();
//    }
}
