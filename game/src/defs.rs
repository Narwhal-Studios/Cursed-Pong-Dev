use serde::{Serialize, Deserialize};
use std::{
    time::Instant,
    process,
};
use whoami::Platform;

#[derive(Debug, Clone, Copy)]
pub enum Page {
    Main,
    Play,
    Exit,
    Installing,
    Rickroll,
    Settings,
    HowToPlay,
    Check,
    Err,
    Confirm,
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
    Install,
    Code(String),
    Check,
    FirstTime,
    Update,
    CheckUp(Page),
}

#[derive(Debug, Clone, Copy)]
pub enum Time {
    Install,
    Update,
}

impl Time {
    pub fn str(&self) -> String {
        match self {
            Self::Install => str("install"),
            Self::Update => str("update"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub code: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    pub id: String,
    pub same: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
    pub same: String,
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

pub fn download() -> String {
    match whoami::platform() {
        Platform::Windows => str("Invoke-WebRequest -URI https://narwhal-studios.github.io/Cursed-Pong/files/files.zip -OutFile ./files.zip"),
        _ => str("wget https://narwhal-studios.github.io/Cursed-Pong/files/files.zip")
    }
}

pub fn str(string: &str) -> String {
    string.to_string()
}
