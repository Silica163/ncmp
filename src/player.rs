use ma_wrapper;
use playlist;
use filelist;

pub enum PlayerCommand {
    Play,
    Pause,
    TogglePause,
    Quit,
    ViewPlaylist,
    ViewFiles { full_path: bool },
    Unknown { cmd: String},
    Empty,
}

pub fn parse_command(user_input: String) -> PlayerCommand {
    let cmd: Vec<&str> = user_input.trim_start().splitn(2, " ").collect();
    match cmd[0] {
        "p"         => PlayerCommand::TogglePause,
        "play"      => PlayerCommand::Play,
        "pause"     => PlayerCommand::Pause,
        "q"         => PlayerCommand::Quit,
        "quit"      => PlayerCommand::Quit,
        "exit"      => PlayerCommand::Quit,
        "playlist"  => PlayerCommand::ViewPlaylist,
        "files"     => PlayerCommand::ViewFiles { full_path: true },
        "f"         => PlayerCommand::ViewFiles { full_path: false},
        ""          => PlayerCommand::Empty,
        cmd         => PlayerCommand::Unknown { cmd: cmd.to_string() } ,
    }
}

pub fn execute_command(
    cmd: PlayerCommand,
    ps: &mut ma_wrapper::PlayerStatus,
    pl: &Vec<playlist::PlaylistItem>,
    files: &Vec<filelist::FileInfo>,
    quit: &mut bool
) {
    match cmd {
        PlayerCommand::Play         => ps.pause = 0,
        PlayerCommand::Pause        => ps.pause = 1,
        PlayerCommand::TogglePause  => ps.pause = !ps.pause,
        PlayerCommand::Quit         => *quit = true,
        PlayerCommand::ViewPlaylist => playlist::show(pl, files),
        PlayerCommand::ViewFiles{full_path}    => filelist::show(files, full_path),
        PlayerCommand::Unknown{cmd} => println!("Unknown command: {cmd}"),
        PlayerCommand::Empty        => {},
    }
}

