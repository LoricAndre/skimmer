use std::sync::Arc;

use rand::{distributions::Alphanumeric, Rng};
use tmux_interface::Tmux;
use tuikit::key::Key;

use crate::{event::Event, SkimItem, SkimOptions, SkimOutput};

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
pub struct TmuxOptions<'a> {
    pub width: &'a str,
    pub height: &'a str,
    pub x: &'a str,
    pub y: &'a str,
}

struct SkimTmuxOutput {
    line: String,
}

impl SkimItem for SkimTmuxOutput {
    fn text(&self) -> &str {
        &self.line
    }
}

impl<'a> From<&'a String> for TmuxOptions<'a> {
    fn from(value: &'a String) -> Self {
        let (raw_dir, size) = value.split_once(",").unwrap_or((value, "50%"));
        let dir = TmuxWindowDir::from(raw_dir);
        let (height, width) = if let Some((lhs, rhs)) = size.split_once(",") {
            match dir {
                TmuxWindowDir::Center | TmuxWindowDir::Left | TmuxWindowDir::Right => (rhs, lhs),
                TmuxWindowDir::Top | TmuxWindowDir::Bottom => (lhs, rhs),
            }
        } else {
            match dir {
                TmuxWindowDir::Left | TmuxWindowDir::Right => ("100%", size),
                TmuxWindowDir::Top | TmuxWindowDir::Bottom => (size, "100%"),
                TmuxWindowDir::Center => (size, size),
            }
        };

        let (x, y) = match dir {
            TmuxWindowDir::Center => ("C", "C"),
            TmuxWindowDir::Top => ("C", "0%"),
            TmuxWindowDir::Bottom => ("C", "100%"),
            TmuxWindowDir::Left => ("0%", "C"),
            TmuxWindowDir::Right => ("100%", "C"),
        };

        Self { height, width, x, y }
    }
}

pub fn run_with(opts: &SkimOptions) -> Option<SkimOutput> {
    // Create temp dir for downstream output
    let temp_dir_name = format!(
        "sk-tmux-{}",
        &rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>(),
    );
    let temp_dir = std::env::temp_dir().join(&temp_dir_name);
    std::fs::create_dir(&temp_dir).unwrap_or_else(|_| panic!("Failed to create temp dir {}", temp_dir.display()));
    let tmp_stdout = temp_dir.join("stdout");
    let tmp_stderr = temp_dir.join("stderr");

    // Build args to send to downstream sk invocation
    let mut tmux_shell_cmd = String::new();
    let mut prev_is_tmux_flag = false;
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

    // Run downstream sk in tmux
    let raw_tmux_opts = &opts.tmux.clone().unwrap();
    let tmux_opts = TmuxOptions::from(raw_tmux_opts);
    let tmux_cmd = tmux_interface::commands::tmux_command::TmuxCommand::new()
        .name("popup")
        .push_flag("-E")
        .push_option("-h", tmux_opts.height)
        .push_option("-w", tmux_opts.width)
        .push_option("-x", tmux_opts.x)
        .push_option("-y", tmux_opts.y)
        .push_param(tmux_shell_cmd)
        .to_owned();

    let status = Tmux::with_command(tmux_cmd)
        .output()
        .expect("Failed to run command in popup")
        .status();

    let output_ending = if opts.print0 { "\0" } else { "\n" };
    let stdout_bytes = std::fs::read_to_string(tmp_stdout).unwrap_or_default();
    let mut stdout = stdout_bytes.split(output_ending);

    let query_str = if opts.print_query && status.success() {
        stdout.next().expect("Not enough lines to unpack in downstream result")
    } else {
        ""
    };

    let command_str = if opts.print_cmd && status.success() {
        stdout.next().expect("Not enough lines to unpack in downstream result")
    } else {
        ""
    };

    let accept_key = if !opts.expect.is_empty() && status.success() {
        Some(
            stdout
                .next()
                .expect("Not enough lines to unpack in downstream result")
                .to_string(),
        )
    } else {
        None
    };

    let mut selected_items: Vec<Arc<dyn SkimItem>> = vec![];
    for line in stdout {
        selected_items.push(Arc::new(SkimTmuxOutput { line: line.to_string() }));
    }

    let is_abort = !status.success();
    let final_event = match is_abort {
        true => Event::EvActAbort,
        false => Event::EvActAccept(accept_key),
    };

    let skim_output = SkimOutput {
        final_event,
        is_abort,
        final_key: Key::Null,
        query: query_str.to_string(),
        cmd: command_str.to_string(),
        selected_items,
    };
    Some(skim_output)
}
