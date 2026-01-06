use hyprgridnavigate::{Side, get_hyprland_command_socket, parse_args};
use std::{
    error::Error,
    io::{Read, Write},
    os::unix::net::UnixStream,
    process,
};

fn main() {
    let args = parse_args().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    change_workspace(args.side, args.move_active_window)
        .unwrap_or_else(|err| eprintln!("App error: {err}"));
}

fn change_workspace(side: Side, move_active_window: bool) -> Result<(), Box<dyn Error>> {
    let socket = get_hyprland_command_socket()?;

    let mut stream = UnixStream::connect(&socket)?;
    stream.write_all("j/activeworkspace".as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    let json: serde_json::Value = serde_json::from_str(&response)?;

    let workspace: &i64 = &json["id"]
        .as_i64()
        .ok_or("Current workspace ID could not be parsed.")?;

    // Math logic to set animation type and workspace target
    let style: &str;
    let mut target: i64 = *workspace;

    match side {
        Side::Top => {
            style = "slidevert";
            if *workspace > 3 {
                target = *workspace - 3;
            }
        }
        Side::Bottom => {
            style = "slidevert";
            if *workspace <= 6 {
                target = *workspace + 3;
            }
        }
        Side::Left => {
            style = "slide";
            if *workspace > 1 {
                target = *workspace - 1;
            }
        }
        Side::Right => {
            style = "slide";
            if *workspace < 9 {
                target = *workspace + 1;
            }
        }
    };

    // Create new stream to write animation and move
    let mut stream = UnixStream::connect(&socket)?;

    stream.write_all(
        format!(
            "[[BATCH]]keyword animation workspaces,1,5,overshot,{};dispatch {} {}",
            style,
            if move_active_window {
                "movetoworkspace"
            } else {
                "workspace"
            },
            target
        )
        .as_bytes(),
    )?;

    // Looking at response is only for development purposes

    // let mut response = String::new();
    // stream.read_to_string(&mut response)?;
    // println!("{response}");

    Ok(())
}
