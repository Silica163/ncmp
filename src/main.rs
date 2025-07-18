use std::env;
use std::thread;
use std::time;
use std::process;
use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::collections::BTreeMap;
use std::collections::VecDeque;

pub mod ma_wrapper;
pub mod player;
pub mod playlist;
pub mod filelist;
pub mod queue;
pub mod history;


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
    let mut input_path: Vec<String> = vec![];
    for i in 1..args.len() {
        input_path.push(args[i].clone());
    }

    let mut audio_files: BTreeMap<usize, FileInfo> = BTreeMap::new();
    scan_and_sort_path(input_path, &mut audio_files);

    let mut pl = playlist::shuffle(&mut audio_files);
//    println!("{pl:?}");

    let mut player_status = ma_wrapper::PlayerStatus { playing: 0, ended: 0, pause: 0, };
    ma_wrapper::init(&player_status);

    let command_avaliable = Arc::new(Mutex::new(false));
    let (cmd_tx, cmd_rx) = channel::<player::Command>();
    let (quit_tx, quit_rx) = channel::<bool>();
    {
        let thread_command_avaliable = Arc::clone(&command_avaliable);
        let mut quit = false;
        thread::spawn(move || {
            let mut input = String::new();
            while !quit {
                print!("> "); io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let cmd = player::parse_command(input.trim().to_string());
                input.clear();
                cmd_tx.send(cmd).unwrap();
                *thread_command_avaliable.lock().unwrap() = true;
                quit = quit_rx.recv().unwrap();
            }
        });
    }

    let mut song: filelist::FileInfo = filelist::FileInfo::new(String::new());
    let mut current_file_idx = 0;
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut hist: VecDeque<usize> = VecDeque::new();
    'outer: while player::next(
        &audio_files,
        &mut song, &mut current_file_idx,
        &mut pl,
        &mut q,
    ){
        println!("Playing: {}", song.name.clone());
        ma_wrapper::play(song.path.clone());
        let mut go_previous = false;
        while !ma_wrapper::is_ended() {
            if *command_avaliable.lock().unwrap() {
                *command_avaliable.lock().unwrap() = false;
                match player::execute_command(
                    cmd_rx.recv().unwrap(),
                    &mut player_status,
                    &mut pl,
                    &mut q,
                    &mut hist,
                    &mut audio_files, current_file_idx
                ) {
                    // TODO: merge quit_tx.send(false) together
                    player::CommandInterrupt::None    => quit_tx.send(false).unwrap(),
                    player::CommandInterrupt::Quit    => {
                        quit_tx.send(true).unwrap();
                        break 'outer;
                    },
                    player::CommandInterrupt::Next    => { quit_tx.send(false).unwrap(); break },
                    player::CommandInterrupt::Previous=> { quit_tx.send(false).unwrap(); go_previous = true; break },
                }
            }
            sleep!(100);
        }
        if !go_previous {
            history::add(&mut hist, current_file_idx);
        }
    }
    try_exit();
}
