use std::collections::BTreeMap;
use std::collections::VecDeque;
use ma_wrapper;
use playlist;
use filelist;
use queue;
use history;

pub enum Command {
    // player
    Play,
    Pause,
    TogglePause,
    Seek { target_sec: i32 },
    Info,
    Quit,

    // Queue
    QueueAdd { file_idxs: Vec<usize> },
    QueueRemove { with_index: bool, index: usize },
    QueueMove { from: usize, to: usize },
    ViewQueue,

    // History
    Next,
    Previous,
    ViewHistory,

    // playlist/files
    ViewPlaylist,
    ViewFiles { full_path: bool },
    RemoveFileById { id: usize },
    RemoveFileByPattern { pattern: String },

    // other
    Help, // XXX: help <command>?
    Unknown { cmd: String, cmd_raw: String },
    Error { msg: String },
    Empty,
}

fn parse_remove_command(cmd: &Vec<&str>) -> Command {
    if cmd.len() < 2 {
        return Command::Error {
            msg: format!("Expect at least one argument, but nothing is provided."),
        }
    }
    match cmd[1].parse::<usize>() {
        Ok(id)  => Command::RemoveFileById { id },
        Err(..) => Command::Error {
            msg: format!("Expect number but got `{}`", cmd[1]),
        },
    }
}

fn parse_remove_pattern_command(cmd: &Vec<&str>) -> Command {
    if cmd.len() < 2 {
        return Command::Error {
            msg: format!("Expect at least one argument, but nothing is provided."),
        }
    }
    Command::RemoveFileByPattern { pattern: cmd[1].to_string() }
}

fn parse_seek_command(cmd: &Vec<&str>) -> Command {
    if cmd.len() < 2 {
        return Command::Error {
            msg: format!("Expect at least one argument, but nothing is provided."),
        }
    }
    match cmd[1].parse::<i32>() {
        Ok(target_sec) if target_sec >= 0 => Command::Seek { target_sec },
        Ok(_) | Err(..) => Command::Error {
            msg: format!("Expect positive number but got `{}`", cmd[1]),
        },
    }
}

fn parse_dequeue_command(cmd: &Vec<&str>) -> Command {
    let mut queue_idx = 0;
    let mut with_index = false;
    if cmd.len() < 2 {
        return Command::QueueRemove { with_index, index: queue_idx };
    }

    with_index = true;
    let args = cmd[1].split(" ").collect::<Vec<&str>>();
    match args[0].parse::<usize>() {
        Ok(n)   => queue_idx = n,
        Err(..) => return Command::Error {
            msg: format!("Expect positive number but got `{}`", args[0]),
        },
    }
    return Command::QueueRemove { with_index, index: queue_idx }
}

fn parse_enqueue_command(cmd: &Vec<&str>) -> Command {
    if cmd.len() < 2 {
        return Command::Error {
            msg: format!("Expect at least one argument, but nothing is provided."),
        }
    }

    let mut files: Vec<usize> = vec![];
    for arg in cmd[1].split(" ") {
        match arg.parse::<usize>() {
            Ok(n)   => files.push(n),
            Err(..) => return Command::Error {
                msg: format!("Expect positive number but got `{}`", arg),
            },
        }
    }

    Command::QueueAdd{ file_idxs: files }
}

fn parse_movequeue_command(cmd: &Vec<&str>) -> Command {
    if cmd.len() != 2 {
        return Command::Error {
            msg: format!("Expect 2 argument, but nothing provided."),
        }
    }

    let from;
    let to;
    {
        let args = cmd[1].split(" ").collect::<Vec<&str>>();
        if args.len() != 2 {
            return Command::Error {
                msg: format!("Expect 2 argument, but {} is given.", args.len()),
            }
        }
        match args[0].parse::<usize>() {
            Ok(n)   => from = n,
            Err(..) => return Command::Error {
                msg: format!("Expect number but got `{}`", args[0]),
            },
        }
        match args[1].parse::<usize>() {
            Ok(n)   => to = n,
            Err(..) => return Command::Error {
                msg: format!("Expect number but got `{}`", args[0]),
            },
        }
    }

    Command::QueueMove{ from, to }
}

type ArgumentParser = fn (&Vec<&str>) -> Command;
const COMMAND_STR_FUNCTION: [(&'static str, ArgumentParser); 20] = [
    ("play",    |_| Command::Play),
    ("pause",   |_| Command::Pause),
    ("p",       |_| Command::TogglePause),
    ("seek",    parse_seek_command),
    ("info",    |_| Command::Info),
    ("quit",    |_| Command::Quit),
    ("enqueue",     parse_enqueue_command),
    ("dequeue",     parse_dequeue_command),
    ("movequeue",   parse_movequeue_command),
    ("queue",       |_| Command::ViewQueue),

    ("next",        |_| Command::Next),
    ("previous",    |_| Command::Previous),
    ("history",     |_| Command::ViewHistory),

    ("playlist",    |_| Command::ViewPlaylist),
    ("files",       |_| Command::ViewFiles { full_path: true }),
    ("f",           |_| Command::ViewFiles { full_path: false }), // TODO: move this to alias
    ("remove",          parse_remove_command),
    ("remove_pattern",  parse_remove_pattern_command),
    ("help",        |_| Command::Help),


    ("",        |_| Command::Empty),
];

const COMMAND_ALIAS: [(&'static str, &'static str); 13] = [
    ("q",   "quit"),
    ("exit","quit"),
    ("enq", "enqueue"),
    ("deq", "dequeue"),
    ("mvq", "movequeue"),
    ("n",   "next"),
    ("prev","previous"),
    ("hist","history"),
    ("r",   "remove"),
    ("rp",  "remove_pattern"),
    ("i",   "info"),
    ("h",   "help"),
    ("s",   "seek"),
];

pub fn parse_command(user_input: String) -> Command {
    let cmd_raw: Vec<&str> = user_input.trim_start().splitn(2, " ").collect();
    let mut cmd = cmd_raw[0];
    for (alias, real_cmd) in COMMAND_ALIAS {
        if alias == cmd {
            println!("{cmd} {real_cmd} {alias}");
            cmd = real_cmd;
            break;
        }
    }
    for (cmd_str, run) in COMMAND_STR_FUNCTION {
        if cmd_str == cmd {
            println!("{cmd} {cmd_raw:?} {cmd_str}");
            return run(&cmd_raw);
        }
    }
    Command::Unknown { cmd: cmd.to_string(), cmd_raw: cmd_raw[0].to_string() }
}

pub enum CommandInterrupt {
    Next,
    Previous,
    Quit,
    None,
}

pub fn execute_command(
    cmd: Command,
    ps: &mut ma_wrapper::PlayerStatus,
    pl: &mut VecDeque<usize>,
    q: &mut VecDeque<usize>,
    hist: &mut VecDeque<usize>,
    files: &mut BTreeMap<usize, filelist::FileInfo>,
    current_file_idx: usize,
) -> CommandInterrupt {
    match cmd {
        Command::Play         => {
            ps.pause = 0;
            CommandInterrupt::None
        },
        Command::Pause        => {
            ps.pause = 1;
            CommandInterrupt::None
        },
        Command::TogglePause  => {
            ps.pause = !ps.pause;
            CommandInterrupt::None
        },
        Command::Seek{target_sec} => {
            ma_wrapper::seek_to_sec(target_sec);
            CommandInterrupt::None
        },
        Command::Info         => {
            info(ps, current_file_idx, files);
            CommandInterrupt::None
        },
        Command::Quit         => CommandInterrupt::Quit,

        Command::QueueAdd { file_idxs } => {
            queue::enqueue_many(q, file_idxs, files);
            CommandInterrupt::None
        },
        Command::QueueRemove { with_index, index } => {
            let mut queue_index = 0;
            if with_index {
                queue_index = index;
            }
            match queue::dequeue_at(q, queue_index) {
                Some(_) => {},
                None    => println!("couldn't remove queue {queue_index}."),
            }
            CommandInterrupt::None
        },
        Command::QueueMove { from, to } => {
            match Some(q.len()) {
                Some(0) => println!("could not move from {from} to {to}: queue is empty."),
                Some(1) => println!("could not move from {from} to {to}: there only one song in queue."),
                Some(x) if x <= from => println!("could not move from {from}: index must less than queue size."),
                Some(x) if x <= to   => println!("could not move to {to}: index must less than queue size."),
                Some(_) => {
                    match queue::dequeue_at(q, from) {
                        Some(file_idx) => queue::enqueue_at(q, to, file_idx, files),
                        None => unreachable!("{}:{}: This should not happen", file!(), line!()),
                    };
                },
                None    => unreachable!("{}:{}: Some(q.len()) == None, something is wrong.", file!(), line!()),
            }
            CommandInterrupt::None
        },
        Command::ViewQueue => {
            show(q, files, "queue");
            CommandInterrupt::None
        },

        Command::Next         => {
            ps.pause = 1;
            CommandInterrupt::Next
        },
        Command::Previous     => {
            let mut last_file_idx = 0;
            if !history::get_and_pop(hist, &mut last_file_idx) {
                println!("Couldn't get previous song: history is empty.");
                CommandInterrupt::None
            } else {
                let can_enqueue_current_file = !queue::enqueue_at(q, 0, current_file_idx, files);
                if !can_enqueue_current_file {
                    println!("file id {current_file_idx:3} does not exist in filelist.");
                }

                // If file in history does not in the list, just ignore previous command.
                if !queue::enqueue_at(q, 0, last_file_idx, files){
                    println!("file id {last_file_idx:3} does not exist in filelist.");

                    if can_enqueue_current_file {
                        // If the current file is already added to the queue, but previous file is not in song list,
                        // remove added file to prevent it from playing again when current file ends.
                        queue::dequeue_at(q, 0);
                    }
                    CommandInterrupt::None
                } else {
                    ps.pause = 1;
                    CommandInterrupt::Previous
                }
            }
        },
        Command::ViewHistory  => {
            show(hist, files, "history");
            CommandInterrupt::None
        },

        Command::ViewPlaylist => {
            show(pl, files, "playlist");
            CommandInterrupt::None
        },
        Command::ViewFiles{full_path} => {
            filelist::show(files, full_path);
            CommandInterrupt::None
        },
        Command::RemoveFileById{id}   => {
            filelist::remove(files, id);
            update(pl, files);
            update(q, files);
            CommandInterrupt::None
        },
        Command::RemoveFileByPattern{pattern} => {
            filelist::remove_by_pattern(files, pattern);
            CommandInterrupt::None
        },

        Command::Help => {
            println!("{:=<25}","");
            for (command, _) in COMMAND_STR_FUNCTION {
                if command != "" { println!("{command}") }
            }
            println!("{:=<25}","");
            CommandInterrupt::None
        },
        Command::Unknown{cmd, cmd_raw} => {
            if cmd == cmd_raw {
                println!("Unknown command: {cmd}");
            } else {
                for (cmd_alias, cmd_real) in COMMAND_ALIAS {
                    if cmd_alias == cmd_raw {
                        println!("Unknown command: \"{cmd_alias}\", which is alias for non existing \"{cmd_real}\" command.");
                        break;
                    }
                }
            }
            CommandInterrupt::None
        },
        Command::Error{msg}   => {
            println!("Error: {msg}");
            CommandInterrupt::None
        },
        Command::Empty        => CommandInterrupt::None,
    }
}

// return false when playlist and queue ended
pub fn next(
    files: &BTreeMap<usize, filelist::FileInfo>,
    out_file: &mut filelist::FileInfo,
    out_file_idx: &mut usize,
    pl: &mut VecDeque<usize>,
    q: &mut VecDeque<usize>,
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

fn show(vdq: &VecDeque<usize>, files: &BTreeMap<usize, filelist::FileInfo>, s: &str) {
    println!("=========={:^10}==========", s);
    for (index, file_idx) in vdq.iter().enumerate() {
        match files.get(&file_idx) {
            Some(file) => println!("{index:03}: {}", file.name),
            None => { println!("file id {file_idx:03} is not exists in file list.")},
        }
    }
    println!("==============================");
}

fn update(vdq: &mut VecDeque<usize>, files: &BTreeMap<usize, filelist::FileInfo>){
    for (index, file_idx) in vdq.clone().iter().enumerate() {
        match files.get(&(file_idx)) {
            Some(_) => {},
            None => { vdq.remove(index); break; },
        }
    }
}
