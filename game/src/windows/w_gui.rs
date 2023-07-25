use crate::colors::{Color, Theme};
use crate::gui_parts::{GuiParts, Position, Velocity};
use iced::time;
use iced::{
    executor,
    theme::Theme as itheme,
    widget::{button, column, container, row, slider, text, Image},
    window, Application, Command, Element,
    Length::Fill,
    Subscription,
};
use rand::{thread_rng, Rng};
use random_word::gen;
use raster::Image as RImage;
use std::{
    process,
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
        self.velocity.x = -self.velocity.x;
    }
    fn changey(&mut self) {
        self.velocity.y = -self.velocity.y;
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
        let text_num = thread_rng().gen_range(0..=8);
        let off_num;
        loop {
            let num = thread_rng().gen_range(0..=8);
            if num != text_num {
                off_num = num;
                break;
            }
        }
        let theme = Theme::new(background, colors[text_num], colors[off_num]);
        let (on, off) = theme.to_rcolor();
        let mut images = (RImage::blank(1, 1), RImage::blank(1, 1));
        images.0.set_pixel(0, 0, on).expect("Failed to set pixel");
        images.1.set_pixel(0, 0, off).expect("Failed to set pixel");
        let mut commds = [
            process::Command::new(format!("rm -r {}Cursed-Pong", home())),
            process::Command::new(format!("mkdir {}Cursed-Pong", home())),
        ];
        commds[0].status().expect("Failed to run command 1");
        commds[1].status().expect("Failed to run command 2");
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
    Rickroll,
    Settings,
    HowToPlay,
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
}

fn home() -> String {
    let platform = whoami::platform();
    let username = whoami::username();
    let double = String::from("/");
    dbg!(double);
    match platform {
        Platform::Windows => format!("C:/Users/{}/AppData/Roaming/", username),
        Platform::Linux => format!("/home/{}/.", username),
        Platform::MacOS => format!("/Users/{}/Library/Application Support/", username),
        _ => {
            rmdir();
            process::exit(1);
        }
    }
}
fn rmdir() {
    let mut commd = process::Command::new(format!("rm -r {}Cursed-Pong", home()));
    commd.status().expect("Failed to run command post");
}
fn windows() -> bool {
    if whoami::platform() == Platform::Windows {
        true
    } else {
        false
    }
}

impl Application for Gui {
    type Message = GuiMessage;
    type Executor = executor::Default;
    type Theme = itheme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<GuiMessage>) {
        let path = format!("{}Cursed-Pong", home());
        println!("{}", path);
        dbg!(path);
        let mut commd = process::Command::new(format!("mkdir {}Cursed-Pong", home()));
        dbg!(&commd);
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
            position: Position::new(5, 4),
            is_playing: false,
            theme: Theme::new(Color::White, Color::Blue, Color::Green).to_theme(),
            ivalue,
            onw: "on".to_string(),
            offw: "off".to_string(),
            score: 0,
        };

        bord.toggle_theme();
        bord.cre_bord();

        (bord, window::change_mode(window::Mode::Fullscreen))
    }
    fn title(&self) -> String {
        "Cursed Pong".to_string()
    }
    fn update(&mut self, message: Self::Message) -> Command<GuiMessage> {
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
                };

                bord.toggle_theme();
                bord.cre_bord();

                *self = bord;
            }
            GuiMessage::Exit => {
                rmdir();
                process::exit(0);
            }
        }
        Command::none()
    }
    fn subscription(&self) -> Subscription<GuiMessage> {
        if self.is_playing {
            time::every(Duration::from_millis(self.delay * 250)).map(GuiMessage::Tick)
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
                        .width(50)
                        .height(50)
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

        match self.page {
            Page::Main => main,
            Page::Play => play,
            Page::Rickroll => rickroll,
            Page::Settings => settings,
            Page::HowToPlay => howtoplay,
            Page::Exit => exit,
        }
    }
    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}
