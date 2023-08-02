use crate::colors::{Color, Theme};
use crate::gui_parts::{GuiParts, Position, Velocity};
use crate::defs::{
    GuiMessage,
    Page,
    Time,
    Update,
    Id,
    Version,
    home,
    sw,
    download,
    str,
};
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
use std::{
    path::Path,
    fs::{self, File},
    process,
    time::Duration,
};
use version::version;
use run_script::run_script;

pub struct Gui {
    pub value: [[bool; 34]; 34],
    pub velocity: Velocity,
    pub delay: u64,
    pub page: Page,
    pub bat_y: usize,
    pub position: Position,
    pub is_playing: bool,
    pub theme: itheme,
    pub ivalue: Vec<Vec<String>>,
    pub onw: String,
    pub offw: String,
    pub score: usize,
    pub code: String,
    pub err: String,
    pub codeu: String,
    pub db: Database,
    pub time: Time,
    pub id: usize,
    pub size: usize,
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
        let path = &format!("{}Cursed-Pong{}white.png", home(), sw().1);
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
            position: Position::new(16, 17),
            is_playing: false,
            theme: Theme::new(Color::White, Color::Blue).to_theme(),
            ivalue,
            onw: str("blue.png"),
            offw: str("white.png"),
            score: 0,
            code: String::new(),
            codeu: String::new(),
            db: client.database("db"),
            id: 0,
            time: Time::Install,
            size: 50,
        };
        bord.cre_bord();
        bord.draw_bat();
        bord.show_pixel();

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
                self.is_playing = false;
                self.page = page;
            }
            GuiMessage::Delay(delay) => self.delay = delay as u64,
            GuiMessage::Code(code) => self.code = code,
            GuiMessage::CheckUp(page) => {
                let path = Path::exists(Path::new(&format!("{}Cursed-Pong", home())));
                match page {
                    Page::Play => {
                        if path {
                            self.page = Page::Play;
                            self.is_playing = true;
                        } else {
                            self.err = "Please install Cursed Pong before playing".to_string();
                            self.page = Page::Err;
                        }
                    }
                    Page::Check => {
                        if path {
                            let version = fs::read_to_string(format!("{}Cursed-Pong{}version.scff", home(), sw().1)).expect("Failed to get version");
                            let versions = self.db.collection::<Version>("version");
                            let cursor = versions.find(doc! { "same": "" }, None).expect("Failed to get most recent version");
                            let mut version_r = String::new();
                            for result in cursor {
                                version_r = result.expect("Failed to get result").version;
                            }
                            if version.trim() == version_r {
                                self.size = 11;
                                self.err = "No new update is available.".to_string();
                                self.page = Page::Err;
                            } else {
                                self.size = 11;
                                self.err = "A new update is available. Click install files on the main page to install.".to_string();                            self.page = Page::Err;
                            }
                        } else {
                            self.err = "Please install Cursed Pong before checking for updates".to_string();
                        }
                    }
                    Page::Confirm => {
                        if path {
                            self.time = Time::Update;
                            self.page = Page::Confirm;
                        } else {
                            self.time = Time::Install;
                            self.page = Page::Check;
                        }
                    }
                    _ => {
                        println!("Nothing");
                    }
                }
            }
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
                    codes.delete_one(doc! { "code": &self.code }, None).expect("Failed to delete code");
                } else {
                    self.err = "Invalid code".to_string();
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
                        fs::create_dir(format!("{}Cursed-Pong", home())).expect("Failed to create directory");
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
                        File::create(format!("{}Cursed-Pong{}id.scff", home(), sw().1))
                            .expect("Failed to create file");
                        fs::write(format!("{}Cursed-Pong{}id.scff", home(), sw().1), text.to_string())
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
                    let text = fs::read_to_string(format!("{}Cursed-Pong{}id.scff", home(), sw().1))
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
                let (code, _, _) = run_script!(format!(r#"
                cd {}Cursed-pong;
                {};
                tar -xzvf files.zip;
                rm files.zip;
                exit 0;
                "#, home(), download())).expect("Failed to run script");
                if code != 0 {
                    panic!("Failed to run installation script");
                }
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

                if self.checkx() && self.checky() {
                    self.onw = str("images/dvd.png");
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
                let db = self.db.clone();
                let mut bord = Self {
                    value: [[false; 34]; 34],
                    velocity: Velocity::new(1, 1),
                    delay: 2,
                    page: Page::Main,
                    bat_y: 8,
                    position: Position::new(16, 17),
                    is_playing: false,
                    theme: Theme::new(Color::White, Color::Blue).to_theme(),
                    ivalue,
                    onw: str("blue.png"),
                    offw: str("white.png"),
                    score: 0,
                    err: String::new(),
                    code: String::new(),
                    codeu: String::new(),
                    db,
                    id: 0,
                    time: Time::Install,
                    size: 50,
                };
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
            button("Play").on_press(GuiMessage::CheckUp(Page::Play)),
            button("How to Play").on_press(GuiMessage::Change(Page::HowToPlay)),
            button("Settings").on_press(GuiMessage::Change(Page::Settings)),
            button("Install files").on_press(GuiMessage::CheckUp(Page::Confirm)),
            button("Check for updates").on_press(GuiMessage::CheckUp(Page::Check)),
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
                        .width(15)
                        .height(15)
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
            Image::new(format!("{}/Cursed-Pong/bluescreen.png", home())),
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
            text(format!("Are you sure you want to {} Cursed Pong?", self.time.str())),
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

        let size = if self.size == 50 {
            50
        } else {
            11
        };

        let err = container(column![
            text(&self.err).size(size),
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
        }
    }
    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}
