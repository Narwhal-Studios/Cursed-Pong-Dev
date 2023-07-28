use crate::colors::{Color, Theme};
use crate::gui_parts::{GuiParts, Position, Velocity};
use iced::time;
use iced::{
    executor,
    theme::Theme as itheme,
    widget::{button, column, container, row, slider, text, text_input, Image},
    window, Application, Element,
    Length::Fill,
    Subscription,
};
use mongodb::{
    bson::{doc, Document},
    error::Error,
    sync::{Client, Database},
};
use rand::{thread_rng, Rng};
use random_word::gen;
use raster::Image as RImage;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    process::{self, Command},
    time::{Duration, Instant},
};
use version::version;
use whoami::Platform;

pub struct Gui {
    value: [[bool; 34]; 34],
    velocity: Velocity,
    delay: u64,
    page: Page,
    bat_y: usize,
    position: Position,
    is_playing: bool,
    theme: itheme,
    ivalue: Vec<Vec<String>>,
    onw: String,
    offw: String,
    score: usize,
    code: String,
    err: String,
    codeu: String,
    db: Database,
    time: Time,
    id: usize,
    version: String,
}

impl GuiParts for Gui {
    fn draw_bat(&mut self) {
        self.value[self.bat_y - 1][1] = true;
        self.value[self.bat_y][1] = true;
        self.value[self.bat_y + 1][1] = true;
        self.ivalue[self.bat_y - 2][0] = format!("{}Cursed-Pong{}{}.png", home(), '/', &self.onw);
        self.ivalue[self.bat_y - 1][0] = format!("{}Cursed-Pong{}{}.png", home(), '/', &self.onw);
        self.ivalue[self.bat_y][0] = format!("{}Cursed-Pong{}{}.png", home(), '/', &self.onw);
    }

    fn clear(&mut self) {
        self.value = [[false; 34]; 34];
        let path = &format!("{}Cursed-Pong{}{}.png", home(), '/', self.offw);
        let mut ivalue = vec![];
        for _ in 0..32 {
            let mut to_add = vec![];
            for _ in 0..32 {
                to_add.push(path.to_string());
            }
            ivalue.push(to_add);
        }
        self.ivalue = ivalue;
        self.cre_bord();
    }
    fn cre_bord(&mut self) {
        self.value[0] = [true; 34];
        self.value[33] = [true; 34];
        for num in 0..34 {
            self.value[num][0] = true;
            self.value[num][33] = true;
        }
    }
    fn show_pixel(&mut self) {
        self.value[self.position.y][self.position.x] = true;
        self.ivalue[self.position.y - 1][self.position.x - 1] =
            format!("{}Cursed-Pong{}{}.png", home(), '/', &self.onw);
    }
    fn check_bat(&self) -> bool {
        if self.position.y == self.bat_y
            || self.position.y == self.bat_y - 1
            || self.position.y == self.bat_y + 1
        {
            if self.velocity.x < 0 {
                if self.position.x == 2 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    fn addx(&mut self) {
        self.position.x = ((self.position.x as i32) + self.velocity.x) as usize;
    }
    fn addy(&mut self) {
        self.position.y = ((self.position.y as i32) + self.velocity.y) as usize;
    }
    fn changex(&mut self) {
        let num = rand::thread_rng().gen_range(0..3);
        if num == 0 {
            self.velocity.x = -self.velocity.x;
        } else if num == 1 {
            self.velocity.x = -self.velocity.x;
            self.velocity.y = -1;
        } else {
            self.velocity.x = -self.velocity.x;
            self.velocity.y = 1;
        }
    }
    fn changey(&mut self) {
        let num = rand::thread_rng().gen_range(0..3);
        if num == 0 {
            self.velocity.y = -self.velocity.y;
        } else if num == 1 {
            self.velocity.y = -self.velocity.y;
            self.velocity.x = -1;
        } else {
            self.velocity.y = -self.velocity.y;
            self.velocity.x = 1;
        }
    }
    fn toggle_theme(&mut self) {
        let colors = [
            Color::Black,
            Color::White,
            Color::Red,
            Color::Orange,
            Color::Yellow,
            Color::Green,
            Color::Blue,
            Color::Purple,
            Color::Pink,
        ];
        let background = colors[thread_rng().gen_range(0..=8)];
        let text_num = colors[thread_rng().gen_range(0..=8)];
        let theme = Theme::new(background, text_num, background);
        let (on, off) = theme.to_rcolor();
        let mut images = (RImage::blank(1, 1), RImage::blank(1, 1));
        images.0.set_pixel(0, 0, on).expect("Failed to set pixel");
        images.1.set_pixel(0, 0, off).expect("Failed to set pixel");
        let mut commd = Command::new("rm");
        commd.arg(format!("{}Cursed-Pong/*", home()));
        commd.status().expect("Failed to run command 1");
        let words = [gen(), gen()];
        raster::save(
            &images.0,
            &format!("{}Cursed-Pong{}{}.png", home(), '/', words[0]),
        )
        .expect("Failed to save image");
        raster::save(
            &images.1,
            &format!("{}Cursed-Pong{}{}.png", home(), '/', words[1]),
        )
        .expect("Failed to save image");
        self.onw = words[0].to_string();
        self.offw = words[1].to_string();
        self.theme = theme.to_theme();
    }
}

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
    FirstTime,
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
    Version,
}

#[derive(Debug, Clone, Copy)]
pub enum Time {
    Install,
    Update,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    code: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    id: String,
    same: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    version: String,
    same: String,
}

fn home() -> String {
    let platform = whoami::platform();
    let username = whoami::username();
    match platform {
        Platform::Windows => format!("C:/Users/{}/AppData/Roaming/", username),
        Platform::Linux => format!("/home/{}/.", username),
        Platform::MacOS => format!("/Users/{}/Library/Application Support/", username),
        _ => process::exit(1),
    }
}

impl Application for Gui {
    type Message = GuiMessage;
    type Executor = executor::Default;
    type Theme = itheme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<GuiMessage>) {
        let client = Client::with_uri_str(
            "mongodb+srv://cursedpong:noddycallum@cursed-pong.4rpcoc2.mongodb.net/",
        )
        .expect("Failed to get client");
        let path = format!("{}Cursed-Pong", home());
        println!("{}", path);
        let mut commd = Command::new("mkdir");
        commd.arg(format!("{}Cursed-Pong", home()));
        commd.status().expect("Failed to run command pre");
        let path = &format!("{}Cursed-Pong{}off.png", home(), '/');
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
            err: String::new(),
            position: Position::new(5, 4),
            is_playing: false,
            theme: Theme::new(Color::White, Color::Blue, Color::Green).to_theme(),
            ivalue,
            onw: "on".to_string(),
            offw: "off".to_string(),
            score: 0,
            code: String::new(),
            codeu: String::new(),
            db: client.database("db"),
            id: 0,
            time: Time::Install,
            version: String::new(),
        };

        bord.toggle_theme();
        bord.cre_bord();

        (bord, window::change_mode(window::Mode::Fullscreen))
    }
    fn title(&self) -> String {
        "Cursed Pong".to_string()
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<GuiMessage> {
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
            GuiMessage::Change(page) => {
                match page {
                    Page::Play => self.is_playing = true,
                    _ => self.is_playing = false,
                }
                self.page = page;
            }
            GuiMessage::Delay(delay) => self.delay = delay as u64,
            GuiMessage::Code(code) => self.code = code,
            GuiMessage::Check => {
                let banned = self.db.collection::<Document>("banned");
                let banned_cursor = banned.find(doc! { "code": &self.code }, None).expect("Failed to get banned codes");
                let banned_len = banned_cursor.collect::<Vec<Result<Document, Error>>>().len();
                let codes = self.db.collection::<Document>("codes");
                let codes_cursor = codes
                    .find(doc! { "code": &self.code }, None)
                    .expect("Failed to get codes");
                let codes_len = codes_cursor.collect::<Vec<Result<Document, Error>>>().len();
                if  codes_len > 0 && banned_len == 0 {
                    self.codeu = self.code.clone();
                    self.page = Page::Confirm;
                } else {
                    self.err = "Invalid code".to_string();
                    self.page = Page::Err;
                }
            }
            GuiMessage::Version => {
                let fmted = format!("{}Cursed-Pong/version.scff", home());
                let version = fs::read_to_string(format!("{}Cursed-Pong/version.scff", home())).expect("Failed to get version");
                let versions = self.db.collection::<Version>("version");
                let cursor = versions.find(doc! { "same": "" }, None).expect("Failed to get most recent version");
                let mut version_r = String::new();
                for result in cursor {
                    version_r = result.expect("Failed to get result").version;
                }
                if version.trim() == version_r {
                    self.err = "No new update is available.".to_string();
                    self.page = Page::Err;
                } else {
                    self.err = "A new update is available. Click install files on the main page to install.".to_string();
                    self.page = Page::Err;
                }
            }
            GuiMessage::FirstTime => {
                self.time = Time::Install;
                self.page = Page::Check;
            }
            GuiMessage::Update => {
                self.time = Time::Update;
                self.page = Page::Check;
            }
            GuiMessage::Install => {
                self.page = Page::Installing;
                'block1: {
                    'block2: {
                        if let Time::Update = self.time {
                            break 'block2;
                        }
                        let id = self.db.collection::<Id>("id");
                        let update = self.db.collection::<Update>("update");
                        let cursor = id
                            .find(doc! { "same": "" }, None)
                            .expect("Failed to get value");
                        let mut text = "5".to_string();
                        for result in cursor {
                            text = result.expect("Failed to get result").id;
                        }
                        let text = text.parse::<usize>().expect("Failed to parse string");
                        File::create(format!("{}Cursed-Pong/id.scff", home()))
                            .expect("Failed to create file");
                        fs::write(format!("{}Cursed-Pong/id.scff", home()), text.to_string())
                            .expect("Failed to write to file");
                        id.delete_many(doc! {}, None)
                            .expect("Failed to delete data");
                        id.insert_one(
                            Id {
                                id: (text + 1).to_string(),
                                same: "".to_string(),
                            },
                            None,
                        )
                        .expect("Failed to insert data 1 1");
                        update
                            .insert_one(
                                Update {
                                    id: text.to_string(),
                                    code: (&self.code).to_string(),
                                },
                                None,
                            )
                            .expect("Failed to insert data 1 2");
                        break 'block1;
                    }
                    let update = self.db.collection::<Update>("update");
                    let text = fs::read_to_string(format!("{}Cursed-Pong/id.scff", home()))
                        .expect("Failed to read to string");
                    let text = text.parse::<usize>().expect("Failed to parse text");
                    self.id = text;
                    update
                        .insert_one(
                            Update {
                                code: (&self.codeu).to_string(),
                                id: text.to_string(),
                            },
                            None,
                        )
                        .expect("Failed to insert data 2 1");
                }
                let mut commds = [
                    Command::new("wget"),
                    Command::new("bash"),
                    Command::new("rm"),
                ];
                let platform = whoami::platform();
                let file = match platform {
                    Platform::Linux => "linux.sh",
                    Platform::MacOS => "macos.sh",
                    Platform::Windows => "windows.sh",
                    _ => process::exit(1),
                };
                commds[0].arg(format!(
                    "https://narwhal-studios.github.io/Cursed-Pong/files{}",
                    file
                ));
                commds[1].arg(file);
                commds[2].arg(file);
                commds[0].current_dir(format!("{}Cursed-Pong", home()));
                commds[1].current_dir(format!("{}Cursed-Pong", home()));
                commds[2].current_dir(format!("{}Cursed-Pong", home()));
                commds[0]
                    .status()
                    .expect("Failed to download install script");
                commds[1].status().expect("Failed to run install script");
                commds[2].status().expect("Failed to delete install script");
                self.page = Page::Main;
            }
            GuiMessage::Tick(_) => {
                self.clear();
                self.draw_bat();
                self.show_pixel();
                if self.position.x == 1 {
                    self.page = Page::Rickroll;
                    self.is_playing = false;
                }
                let xv = self.value[self.position.y]
                    [((self.position.x as i32) + self.velocity.x) as usize];
                let yv = self.value[((self.position.y as i32) + self.velocity.y) as usize]
                    [self.position.x];

                if xv != false {
                    self.changex();
                    self.toggle_theme();
                    if self.position.x == 2 {
                        self.score += 1;
                    }
                }
                if yv != false {
                    self.changey();
                    self.toggle_theme();
                }

                self.addx();
                self.addy();
            }
            GuiMessage::Restart => {
                let path = &format!("{}Cursed-Pong{}off.png", home(), '/');
                let mut ivalue = vec![];
                for _ in 0..32 {
                    let mut to_add = vec![];
                    for _ in 0..32 {
                        to_add.push(path.to_string());
                    }
                    ivalue.push(to_add);
                }
                let client = Client::with_uri_str(
                    "mongodb+srv://cursedpong:noddycallum@cursed-pong.4rpcoc2.mongodb.net/",
                )
                .expect("Failed to get client");
                let mut bord = Self {
                    value: [[false; 34]; 34],
                    velocity: Velocity::new(1, 1),
                    delay: 2,
                    page: Page::Main,
                    bat_y: 8,
                    position: Position::new(5, 4),
                    is_playing: false,
                    theme: Theme::new(Color::White, Color::Blue, Color::Green).to_theme(),
                    ivalue,
                    onw: "on".to_string(),
                    offw: "off".to_string(),
                    score: 0,
                    err: String::new(),
                    code: String::new(),
                    codeu: String::new(),
                    db: client.database("db"),
                    id: 0,
                    time: Time::Install,
                    version: String::new(),
                };

                bord.toggle_theme();
                bord.cre_bord();

                *self = bord;
            }
            GuiMessage::Exit => process::exit(0),
        }
        iced::Command::none()
    }
    fn subscription(&self) -> Subscription<GuiMessage> {
        if self.is_playing {
            time::every(Duration::from_millis(self.delay * 100)).map(GuiMessage::Tick)
        } else {
            Subscription::none()
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        let main = container(column![
            text(format!("Cursed Pong v{}", version!())).size(50),
            button("Play").on_press(GuiMessage::Change(Page::Play)),
            button("How to Play").on_press(GuiMessage::Change(Page::HowToPlay)),
            button("Settings").on_press(GuiMessage::Change(Page::Settings)),
            button("Install files").on_press(GuiMessage::Change(Page::FirstTime)),
            button("Check for updates").on_press(GuiMessage::Version),
            button("Exit").on_press(GuiMessage::Change(Page::Exit)),
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();
        let mut to_add_bef: Vec<Element<'_, GuiMessage>> = vec![];
        for num1 in 0..32 {
            let mut toadd = vec![];
            for num2 in 0..32 {
                toadd.push(
                    Image::new(&self.ivalue[num1][num2])
                        .width(25)
                        .height(25)
                        .into(),
                );
            }
            to_add_bef.push(row(toadd).into())
        }
        let to_add = column(to_add_bef);
        let play = container(column![
            text(format!("Score: {}", self.score)).size(20),
            to_add,
            row![
                button("Up").on_press(GuiMessage::Up),
                button("Down").on_press(GuiMessage::Down)
            ],
            button("Restart").on_press(GuiMessage::Restart),
            button("Exit").on_press(GuiMessage::Change(Page::Exit)),
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        let rickroll = container(column![
            text("Game Over!").size(50),
            text(format!("Score: {}", self.score)),
            button("Play again").on_press(GuiMessage::Restart)
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        let settings = container(column![
            button("Main Page").on_press(GuiMessage::Change(Page::Main)),
            text(format!("Delay: {}", self.delay)),
            slider(1..=8, self.delay as u8, GuiMessage::Delay),
            button("Exit").on_press(GuiMessage::Change(Page::Exit)),
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        let howtoplay = container(
            column![
                text("How to Play").size(50),
                text("Cursed Pong is an incredibly fun, relaxing game based on the game Pong made by Atari. To play press the up and down buttons to move the paddle to bounce the ball."),
                button("Main Page").on_press(GuiMessage::Change(Page::Main)),
            ]
            ).center_x().center_y().height(Fill).width(Fill).into();

        let exit = container(column![
            text("Exit").size(50),
            text("Are you sure you want to exit?"),
            row![
                button("Exit").on_press(GuiMessage::Exit),
                button("Main Page").on_press(GuiMessage::Restart)
            ],
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();
        let installing = container(text("Installing...").size(50))
            .center_x()
            .center_y()
            .height(Fill)
            .width(Fill)
            .into();

        let check = container(column![
            text("Code").size(50),
            row![
                text("Enter your code: "),
                text_input(" ", &self.code, GuiMessage::Code)
            ],
            row![
                button("Go").on_press(GuiMessage::Check),
                button("Back to main page").on_press(GuiMessage::Change(Page::Main))
            ],
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        let confirm = container(column![
            text("Install").size(50),
            text("Are you sure you want to install Cursed Pong?"),
            row![
                button("Yes").on_press(GuiMessage::Install),
                button("No").on_press(GuiMessage::Change(Page::Main))
            ],
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        let first_time = container(column![
            text("Install").size(50),
            text("Is this the first time you are installing Cursed Pong or is this an update?"),
            row![
                button("First Time").on_press(GuiMessage::FirstTime),
                button("Update").on_press(GuiMessage::Update),
            ],
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        let err = container(column![
            text(&self.err).size(50),
            button("Back to main page").on_press(GuiMessage::Change(Page::Main)),
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        match self.page {
            Page::Main => main,
            Page::Play => play,
            Page::Rickroll => rickroll,
            Page::Settings => settings,
            Page::HowToPlay => howtoplay,
            Page::Installing => installing,
            Page::Exit => exit,
            Page::Check => check,
            Page::Confirm => confirm,
            Page::Err => err,
            Page::FirstTime => first_time,
        }
    }
    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}
