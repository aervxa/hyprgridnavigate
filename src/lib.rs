use std::{env, path::PathBuf, str::FromStr};

#[derive(Debug)]
pub enum Side {
    Left,
    Top,
    Right,
    Bottom,
}

impl FromStr for Side {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "left" => Ok(Side::Left),
            "top" => Ok(Side::Top),
            "right" => Ok(Side::Right),
            "bottom" => Ok(Side::Bottom),
            _ => Err(format!("'{s}' is not a valid side")),
        }
    }
}

pub struct Args {
    pub side: Side,
    pub move_active_window: bool,
}

pub fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("usage: {} <side>", &args[0]));
    };

    let mut move_active_window: bool = false;

    if args.len() == 3 {
        if &args[2] == "true" {
            move_active_window = true;
        }
    }

    match args[1].parse() {
        Ok(side) => Ok(Args {
            side,
            move_active_window,
        }),
        Err(err) => Err(format!("{err}")),
    }
}

pub fn get_hyprland_command_socket() -> Result<PathBuf, String> {
    let runtime_dir = env::var("XDG_RUNTIME_DIR").map_err(|_| "XDG_RUNTIME_DIR not set")?;

    let his = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .map_err(|_| "HYPRLAND_INSTANCE_SIGNATURE not found. Is Hyprland even running?")?;

    // $XDG_RUNTIME_DIR/hypr/[HIS]/.socket.sock
    let mut path = PathBuf::from(runtime_dir);
    path.push("hypr");
    path.push(his);
    path.push(".socket.sock");

    if path.exists() {
        Ok(path)
    } else {
        Err(format!("Socket not found at {:?}", path))
    }
}
