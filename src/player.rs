use std::collections::BTreeMap;
use std::collections::VecDeque;
use ma_wrapper;
use playlist;
use filelist;
use queue;

pub enum PlayerCommand {
    // player
    Play,
    Pause,
    TogglePause,
    Seek { target_sec: i32 },
    Info,
    Quit,

    // Queue
    QueueAdd { with_index: bool, index: usize, file_idx: usize },
    QueueRemove { with_index: bool, index: usize },
    ViewQueue,

    // playlist/files
    ViewPlaylist,
    ViewFiles { full_path: bool },
    RemoveFileById { id: usize },

    // other
    Unknown { cmd: String },
    Error { msg: String },
    Empty,
}

fn parse_remove_command(cmd: &Vec<&str>) -> PlayerCommand {
    if cmd.len() < 2 {
        return PlayerCommand::Error {
            msg: format!("Expect at least one argument, but nothing is provided."),
        }
    }
    match cmd[1].parse::<usize>() {
        Ok(id)  => PlayerCommand::RemoveFileById { id },
        _       => PlayerCommand::Error {
            msg: format!("Expect number but got `{}`", cmd[1]),
        },
    }
}

fn parse_seek_command(cmd: &Vec<&str>) -> PlayerCommand {
    if cmd.len() < 2 {
        return PlayerCommand::Error {
            msg: format!("Expect at least one argument, but nothing is provided."),
        }
    }
    match cmd[1].parse::<i32>() {
        Ok(target_sec)  => PlayerCommand::Seek { target_sec },
        _   => PlayerCommand::Error {
            msg: format!("Expect number but got `{}`", cmd[1]),
        },
    }
}

fn parse_queue_command(cmd: &Vec<&str>, is_enqueue: bool) -> PlayerCommand {
    let mut file_idx = 0;
    let mut queue_idx = 0;
    let mut with_index = false;
    if cmd.len() < 2 {
        if !is_enqueue {
            return PlayerCommand::QueueRemove { with_index, index: queue_idx };
        }
        return PlayerCommand::Error {
            msg: format!("Expect at least one argument, but nothing is provided."),
        }
    }
    let args = cmd[1].split(" ").collect::<Vec<&str>>();

    match args[0].parse::<usize>() {
        Ok(n)   => if is_enqueue { file_idx = n } else { queue_idx = n },
        _       => return PlayerCommand::Error {
            msg: format!("Expect number but got `{}`", args[1]),
        },
    }

    if args.len() > 1 && is_enqueue {
        match args[1].parse::<usize>() {
            Ok(n)   => { with_index = true; file_idx = n; },
            _       => return PlayerCommand::Error {
                msg: format!("Expect number but got `{}`", args[1]),
            },
        }
    }

    if is_enqueue {
        return PlayerCommand::QueueAdd { with_index, index: queue_idx, file_idx }
    } else {
        return PlayerCommand::QueueRemove { with_index, index: queue_idx }
    }
}

pub fn parse_command(user_input: String) -> PlayerCommand {
    let cmd: Vec<&str> = user_input.trim_start().splitn(2, " ").collect();
    match cmd[0] {
        "play"      => PlayerCommand::Play,
        "pause"     => PlayerCommand::Pause,
        "p"         => PlayerCommand::TogglePause,
        "seek"      => parse_seek_command(&cmd),
        "info"      => PlayerCommand::Info,
        "q"         => PlayerCommand::Quit,
        "quit"      => PlayerCommand::Quit,
        "exit"      => PlayerCommand::Quit,

        "enqueue"   => parse_queue_command(&cmd, true),
        "enq"       => parse_queue_command(&cmd, true),
        "dequeue"   => parse_queue_command(&cmd, false),
        "deq"       => parse_queue_command(&cmd, false),
        "queue"     => PlayerCommand::ViewQueue,

        "playlist"  => PlayerCommand::ViewPlaylist,
        "files"     => PlayerCommand::ViewFiles { full_path: true },
        "f"         => PlayerCommand::ViewFiles { full_path: false},
        "remove"    => parse_remove_command(&cmd),
        "r"         => parse_remove_command(&cmd),
        ""          => PlayerCommand::Empty,
        cmd         => PlayerCommand::Unknown { cmd: cmd.to_string() } ,
    }
}

pub fn execute_command(
    cmd: PlayerCommand,
    ps: &mut ma_wrapper::PlayerStatus,
    pl: &mut VecDeque<usize>,
    q: &mut VecDeque<usize>,
    files: &mut BTreeMap<usize, filelist::FileInfo>,
    current_file_idx: usize,
    quit: &mut bool
) {
    match cmd {
        PlayerCommand::Play         => ps.pause = 0,
        PlayerCommand::Pause        => ps.pause = 1,
        PlayerCommand::TogglePause  => ps.pause = !ps.pause,
        PlayerCommand::Seek{target_sec}     => { ma_wrapper::seek_to_sec(target_sec); () }
        PlayerCommand::Info         => info(ps, current_file_idx, files),
        PlayerCommand::Quit         => *quit = true,

        PlayerCommand::QueueAdd { with_index, index, file_idx } => {
            let mut queue_index = q.len();
            if with_index {
                queue_index = index
            }
            if !queue::enqueue_at(q, queue_index, file_idx, files) {
                println!("file id {file_idx:3} does not exist.")
            }
            ()
        },
        PlayerCommand::QueueRemove { with_index, index } => {
            let mut queue_index = 0;
            if with_index {
                queue_index = index;
            }
            if !queue::dequeue_at(q, queue_index) {
                println!("couldn't remove queue {queue_index}.")
            }
            ()
        },
        PlayerCommand::ViewQueue => queue::show(q, files),

        PlayerCommand::ViewPlaylist => playlist::show(pl, files),
        PlayerCommand::ViewFiles{full_path} => filelist::show(files, full_path),
        PlayerCommand::RemoveFileById{id}   => {
            filelist::remove(files, id);
            playlist::update(pl, files);
        },
        PlayerCommand::Unknown{cmd} => println!("Unknown command: {cmd}"),
        PlayerCommand::Error{msg}   => println!("Error: {msg}"),
        PlayerCommand::Empty        => {},
    }
}

// return false when playlist and queue ended
pub fn next(
    files: &BTreeMap<usize, filelist::FileInfo>,
    out_file: &mut filelist::FileInfo,
    out_file_idx: &mut usize,
    pl: &mut VecDeque<usize>,
    q: &mut VecDeque<usize>
) -> bool {
    let mut file_idx = 0;
    if queue::next(q, &mut file_idx) {
        match files.get(&file_idx) {
            Some(file) => {
                *out_file = file.clone();
                *out_file_idx = file_idx;
                return true
            },
            None => {
            },
        }
    }
    while playlist::next(pl, &mut file_idx) {
        match files.get(&file_idx) {
            Some(file) => {
                *out_file = file.clone();
                *out_file_idx = file_idx;
                return true
            },
            None => {
            },
        }
    };
    return false
}

fn info(ps: &ma_wrapper::PlayerStatus, file_idx: usize, files: &mut BTreeMap<usize, filelist::FileInfo>){
    let file = files.get_mut(&file_idx).unwrap();
    // TODO: unwrap will return error when playing song has been remove from filelist.
    if file.length == 0 {
        file.length = ma_wrapper::get_length_in_secs();
    }
    let cursor = ma_wrapper::get_cursor_in_secs();

    println!("==============================");
    println!("status: {}, {cursor:3}/{:3}s", if ps.pause != 0 { "pause" } else { "playing" }, file.length);
    println!("filename: \"{}\"", file.name);
    println!("full_path: \"{}\"", file.path);
    println!("==============================");
}
