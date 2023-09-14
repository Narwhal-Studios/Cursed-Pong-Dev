use crate::colors::{Color, Theme};
use crate::defs::{home, str, GuiMessage, Page, Updates};
use crate::gui::Gui;
use crate::gui_parts::{GuiParts, Position, Velocity};
use mongodb::{
    bson::doc,
    options::ClientOptions,
    sync::{Client, Database},
};
use rand::Rng;
use rusty_audio::Audio;
use std::{
    fs,
    process::{
        self,
        Command,
    }
};
use whoami::Platform;

pub trait Fns {
    fn updatefn(&mut self, message: GuiMessage);
}

impl Fns for Gui {
    fn updatefn(&mut self, message: GuiMessage) {
        match message {
            GuiMessage::Up => {
                if self.bat_y > 2 {
                    self.bat_y -= 1;
                }
            }
            GuiMessage::Down => {
                if self.bat_y < 31 {
                    self.bat_y += 1;
                }
            }
            GuiMessage::Change(page) => match page {
                Page::Play => {
                    self.is_playing = true;
                    self.page = Page::Play;
                }
                _ => {
                    self.is_playing = false;
                    self.page = page;
                }
            },
            GuiMessage::Delay(delay) => self.delay = delay as u64,

            GuiMessage::Tick(_) => {
                if !self.init {
                    let uri =
                        "mongodb+srv://cursedpong:noddycallum@cursed-pong.4rpcoc2.mongodb.net/";
                    let mut client_options = ClientOptions::parse(uri).unwrap();
                    let client = Client::with_options(client_options).unwrap();
                    self.db = Some(client.database("db"));
                    self.page = Page::Main;
                    self.init = true;
                }
                self.clear();
                self.draw_bat();
                self.show_pixel();
                if self.position.x == 1 {
                    self.page = Page::Rickroll;
                    self.is_playing = false;
                    self.audio.play("rickroll");
                }
                if self.check_will() {
                    self.score -= rand::thread_rng().gen_range(1..=20);
                }
                if self.check_rick() {
                    self.score -= rand::thread_rng().gen_range(1..=20);
                }
                let xv = self.value[self.position.y]
                    [((self.position.x as i32) + self.velocity.x) as usize];
                let yv = self.value[((self.position.y as i32) + self.velocity.y) as usize]
                    [self.position.x];

                if xv != false {
                    self.changex();
                    let _ = self.toggle_theme();
                    if self.position.x == 2 {
                        self.score += 1;
                    }
                }
                if yv != false {
                    self.changey();
                    let _ = self.toggle_theme();
                }

                self.addx();
                self.addy();
            }
            GuiMessage::Restart => {
                let path = &format!("{}Cursed-Pong{}white.png", home(), '/');
                let mut ivalue = vec![];
                for _ in 0..32 {
                    let mut to_add = vec![];
                    for _ in 0..32 {
                        to_add.push(path.to_string());
                    }
                    ivalue.push(to_add);
                }
                let mut bord = Self {
                    value: [[false; 34]; 34],
                    velocity: Velocity::new(1, 1),
                    delay: 2,
                    page: Page::Main,
                    bat_y: 8,
                    position: Position::new(16, 17),
                    is_playing: false,
                    theme: Theme::new(Color::White, Color::Blue).to_theme("default"),
                    ivalue,
                    onw: str("images/default/blue.png"),
                    offw: str("images/default/white.png"),
                    score: 0,
                    err: String::new(),
                    id: 0,
                    size: 50,
                    text: String::new(),
                    will: Position::new(13, 23),
                    rick: Position::new(23, 13),
                    texture: (&self.texture).to_string(),
                    texture_temp: str(""),
                    sound: (&self.sound).to_string(),
                    sound_temp: str(""),
                    audio: Audio::new(),
                    init: true,
                    db: self.db.clone(),
                };
                bord.cre_bord();
                bord.draw_bat();
                bord.show_pixel();

                bord.audio.add(
                    "black",
                    &format!("{}Cursed-Pong/audio/default/black.ogg", home()),
                );
                bord.audio.add(
                    "white",
                    &format!("{}Cursed-Pong/audio/default/white.ogg", home()),
                );
                bord.audio.add(
                    "red",
                    &format!("{}Cursed-Pong/audio/default/red.ogg", home()),
                );
                bord.audio.add(
                    "orange",
                    &format!("{}Cursed-Pong/audio/default/orange.ogg", home()),
                );
                bord.audio.add(
                    "yellow",
                    &format!("{}Cursed-Pong/audio/default/yellow.ogg", home()),
                );
                bord.audio.add(
                    "green",
                    &format!("{}Cursed-Pong/audio/default/green.ogg", home()),
                );
                bord.audio.add(
                    "blue",
                    &format!("{}Cursed-Pong/audio/default/blue.ogg", home()),
                );
                bord.audio.add(
                    "purple",
                    &format!("{}Cursed-Pong/audio/default/purple.ogg", home()),
                );
                bord.audio.add(
                    "pink",
                    &format!("{}Cursed-Pong/audio/default/pink.ogg", home()),
                );
                bord.audio.add(
                    "rickroll",
                    &format!("{}Cursed-Pong/audio/default/rickroll.ogg", home()),
                );

                *self = bord;
            }
            GuiMessage::Exit => process::exit(0),
            GuiMessage::Texture(text) => self.texture_temp = text,
            GuiMessage::TextureAssign => self.texture = (&self.texture_temp).to_string(),
            GuiMessage::Sound(text) => self.sound_temp = text,
            GuiMessage::SoundAssign => {
                self.sound = (&self.sound_temp).to_string();
                self.audio.add(
                    "black",
                    &format!("{}Cursed-Pong/audio/{}/black.ogg", home(), self.sound),
                );
                self.audio.add(
                    "white",
                    &format!("{}Cursed-Pong/audio/{}/white.ogg", home(), self.sound),
                );
                self.audio.add(
                    "red",
                    &format!("{}Cursed-Pong/audio/{}/red.ogg", home(), self.sound),
                );
                self.audio.add(
                    "orange",
                    &format!("{}Cursed-Pong/audio/{}/orange.ogg", home(), self.sound),
                );
                self.audio.add(
                    "yellow",
                    &format!("{}Cursed-Pong/audio/{}/yellow.ogg", home(), self.sound),
                );
                self.audio.add(
                    "green",
                    &format!("{}Cursed-Pong/audio/{}/green.ogg", home(), self.sound),
                );
                self.audio.add(
                    "blue",
                    &format!("{}Cursed-Pong/audio/{}/blue.ogg", home(), self.sound),
                );
                self.audio.add(
                    "purple",
                    &format!("{}Cursed-Pong/audio/{}/purple.ogg", home(), self.sound),
                );
                self.audio.add(
                    "pink",
                    &format!("{}Cursed-Pong/audio/{}/pink.ogg", home(), self.sound),
                );
            }
            GuiMessage::CheckUpdates => {
                let mut version_col = self.db.unwrap().collection("version");
                let version = version_col.find_one(doc! {}, None).unwrap().unwrap();
                let newest_num: i32 = version.version.unwrap().parse().unwrap();
                let current_num: i32 = 
                fs::read_to_string(&format!("{}Cursed-Pong/version.ossl", home()))
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();
                if newest_num > current_num {
                    self.page = Page::Updates;
                } else {
                    self.page = Page::NoUpdates;
                }
            }
            GuiMessage::LaunchUpdater => {
                match whoami::platform() {
                    Platform::Windows => {
                        let commd = Command::new(format!("{}Cursed-Pong/updater.exe", home()));
                        commd.status().expect("Failed to run command");
                    }
                    Platform::MacOS => {
                        let commd = Command::new(&format!("open -a {}Cursed-Pong/updater.app", home()));
                        commd.status().expect("Failed to run command");
                    }
                    _ => panic!("yOu TriEd tO TrICk Me DiDnT yOU??!"),
                }
            }
        }
    }
}
