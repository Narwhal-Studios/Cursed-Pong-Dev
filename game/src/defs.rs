use std::{process, time::Instant};
use whoami::Platform;

#[derive(Debug, Clone, Copy)]
pub enum Page {
    Main,
    Play,
    Rickroll,
    Settings,
    HowToPlay,
    Err,
}

#[derive(Debug, Clone)]
pub enum GuiMessage {
    Up,
    Down,
    Change(Page),
    Delay(u8),
    Tick(Instant),
    Exit,
    Restart,
    Texture(String),
    TextureAssign,
}

pub fn home() -> String {
    let platform = whoami::platform();
    let username = whoami::username();
    match platform {
        Platform::Windows => format!("C:/Users/{}/AppData/Roaming/", username),
        Platform::Linux => format!("/home/{}/.", username),
        Platform::MacOS => format!("/Users/{}/Library/Application Support/", username),
        _ => process::exit(1),
    }
}

pub fn sw() -> (bool, char) {
    match whoami::platform() {
        Platform::Windows => (true, '/'),
        _ => (true, '/'),
    }
}

pub fn str(string: &str) -> String {
    string.to_string()
}
