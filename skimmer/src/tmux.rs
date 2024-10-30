use rand::{distributions::Alphanumeric, Rng};
use tmux_interface::{
    commands::common::Size, HasSession, KillSession, NewSession, NewWindow, SplitWindow, StdIO, Tmux,
};

use crate::SkimOptions;

#[derive(Debug)]
enum TmuxWindowDir {
    Center,
    Top,
    Bottom,
    Left,
    Right,
}

impl From<&str> for TmuxWindowDir {
    fn from(value: &str) -> Self {
        match value {
            "center" => TmuxWindowDir::Center,
            "top" => TmuxWindowDir::Top,
            "bottom" => TmuxWindowDir::Bottom,
            "left" => TmuxWindowDir::Left,
            "right" => TmuxWindowDir::Right,
            _ => panic!("Invalid tmux window dir {value}"),
        }
    }
}

#[derive(Debug)]
pub struct TmuxOptions {
    dir: TmuxWindowDir,
    width: Size,
    height: Size,
}
fn str_to_pane_size(value: &str) -> Size {
    match value.chars().last() {
        Some('%') => Size::Percentage(
            value[..value.len() - 1]
                .parse::<usize>()
                .expect("Invalid percentage for tmux arg"),
        ),
        _ => Size::Size(value.parse::<usize>().expect("Invalid row/col count for tmux arg")),
    }
}

impl From<&SkimOptions> for TmuxOptions {
    fn from(value: &SkimOptions) -> Self {
        let raw_opt = value.tmux.clone().unwrap();
        let (raw_dir, size) = raw_opt.split_once(",").unwrap_or((&raw_opt, "50%"));
        let dir = TmuxWindowDir::from(raw_dir);
        if let Some((lhs, rhs)) = size.split_once(",") {
            match dir {
                TmuxWindowDir::Center | TmuxWindowDir::Left | TmuxWindowDir::Right => Self {
                    dir,
                    height: str_to_pane_size(rhs),
                    width: str_to_pane_size(lhs),
                },
                TmuxWindowDir::Top | TmuxWindowDir::Bottom => Self {
                    dir,
                    height: str_to_pane_size(lhs),
                    width: str_to_pane_size(rhs),
                },
            }
        } else {
            match dir {
                TmuxWindowDir::Left | TmuxWindowDir::Right => Self {
                    dir,
                    height: Size::Percentage(100),
                    width: str_to_pane_size(size),
                },
                TmuxWindowDir::Top | TmuxWindowDir::Bottom => Self {
                    dir,
                    height: str_to_pane_size(size),
                    width: Size::Percentage(100),
                },
                TmuxWindowDir::Center => Self {
                    dir,
                    height: str_to_pane_size(size),
                    width: str_to_pane_size(size),
                },
            }
        }
    }
}

pub fn open_with(opts: &TmuxOptions) {
    let mut tmux_shell_cmd = String::new();
    let mut prev_is_tmux_flag = false;
    let temp_dir_name = format!("sk-tmux-{}",
        &rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>(),
    );
    let temp_dir = std::env::temp_dir().join(&temp_dir_name);
    std::fs::create_dir(&temp_dir).expect(&format!("Failed to create temp dir {}", temp_dir.display()));
    let tmp_stdout = temp_dir.join("stdout");
    let tmp_stderr = temp_dir.join("stderr");
    for arg in std::env::args() {
        if prev_is_tmux_flag {
            prev_is_tmux_flag = false;
            if !arg.starts_with("-") {
                continue;
            }
        }
        if arg == "--tmux" {
            prev_is_tmux_flag = true;
            continue;
        }
        tmux_shell_cmd.push_str(&format!(" {arg}"));
    }
    tmux_shell_cmd.push_str(&format!(" >{} 2>{}", tmp_stdout.display(), tmp_stderr.display()));
    Tmux::with_command(
        tmux_interface::commands::DisplayPopup::new()
            .height(opts.height.clone())
            .width(opts.width.clone())
            .shell_command(tmux_shell_cmd)
            .close_on_exit()
            .build(),
    )
    .output()
    .expect("Failed to run command in popup");
}
