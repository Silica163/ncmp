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
    Empty,
}

fn parse_remove_command(cmd: &Vec<&str>) -> PlayerCommand {
    if cmd.len() < 2 { return PlayerCommand::Empty; }
    match cmd[1].parse::<usize>() {
        Ok(id)  => PlayerCommand::RemoveFileById { id },
        _       => {
            println!("Expect number but got `{}`", cmd[1]);
            PlayerCommand::Empty
        },
    }
}

fn parse_seek_command(cmd: &Vec<&str>) -> PlayerCommand {
    if cmd.len() < 2 { return PlayerCommand::Empty; }
    match cmd[1].parse::<i32>() {
        Ok(target_sec)  => PlayerCommand::Seek { target_sec },
        _   => {
            println!("Expect number but got `{}`", cmd[1]);
            PlayerCommand::Empty
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
        return PlayerCommand::Empty;
    }
    let args = cmd[1].split(" ").collect::<Vec<&str>>();

    match args[0].parse::<usize>() {
        Ok(n)   => if is_enqueue { file_idx = n } else { queue_idx = n },
        _       => {
            println!("Expect number but got `{}`", args[0]);
            return PlayerCommand::Empty
        },
    }

    if args.len() > 1 && is_enqueue {
        match args[1].parse::<usize>() {
            Ok(n)   => { with_index = true; file_idx = n; },
            _       => {
                println!("Expect number but got `{}`", args[1]);
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
    pl: &mut Vec<playlist::PlaylistItem>,
    q: &mut VecDeque<queue::QueueItem>,
    files: &mut BTreeMap<usize, filelist::FileInfo>,
    quit: &mut bool
) {
    match cmd {
        PlayerCommand::Play         => ps.pause = 0,
        PlayerCommand::Pause        => ps.pause = 1,
        PlayerCommand::TogglePause  => ps.pause = !ps.pause,
        PlayerCommand::Seek{target_sec}     => { ma_wrapper::seek_to_sec(target_sec); () }
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
            match filelist::remove(files, id){
                Some(idx)   => { pl.remove(idx); },
                None        => {},
            }
        },
        PlayerCommand::Unknown{cmd} => println!("Unknown command: {cmd}"),
        PlayerCommand::Empty        => {},
    }
}

// return false when playlist and queue ended
pub fn next(
    files: &BTreeMap<usize, filelist::FileInfo>,
    out_file: &mut filelist::FileInfo,
    pl: &mut Vec<playlist::PlaylistItem>, pl_current_song: &mut usize,
    q: &mut VecDeque<queue::QueueItem>
) -> bool {
    let mut file_idx = 0;
    if queue::next(q, &mut file_idx) {
        match files.get(&file_idx) {
            Some(file) => {
                *out_file = file.clone();
                return true
            },
            None => {
                pl.remove(*pl_current_song);
                *pl_current_song -= 1;
            },
        }
    }
    while playlist::next(pl, pl_current_song) {
        match files.get(&pl[*pl_current_song].file_idx) {
            Some(file) => {
                *out_file = file.clone();
                return true
            },
            None => {
                pl.remove(*pl_current_song);
                *pl_current_song -= 1;
            },
        }
    };
    return false
}
