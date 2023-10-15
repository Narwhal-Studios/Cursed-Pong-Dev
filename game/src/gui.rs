use crate::colors::{Color, Theme};
use crate::defs::{home, str, sw, GuiMessage, Page, Updates};
use crate::gui_parts::{GuiParts, Position, Velocity};
use crate::updatefn::Fns;
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
    options::ClientOptions,
    sync::{Client, Database},
};
use rusty_audio::Audio;
use std::time::Duration;
use version::version;

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
    pub score: i32,
    pub err: String,
    pub id: usize,
    pub size: usize,
    pub text: String,
    pub will: Position,
    pub rick: Position,
    pub texture: String,
    pub texture_temp: String,
    pub sound: String,
    pub sound_temp: String,
    pub audio: Audio,
    pub init: bkool,
    pub db: Option<Database>,
}

impl Application for Gui {
    type Message = GuiMessage;
    type Executor = executor::Default;
    type Theme = itheme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<GuiMessage>) {
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
            page: Page::Wait,
            bat_y: 8,
            err: String::new(),
            position: Position::new(16, 17),
            is_playing: false,
            theme: Theme::new(Color::White, Color::Blue).to_theme("default"),
            ivalue,
            onw: str("images/default/blue.png"),
            offw: str("images/default/white.png"),
            score: 0,
            id: 0,
            size: 50,
            text: String::new(),
            will: Position::new(13, 23),
            rick: Position::new(23, 13),
            texture: str("default"),
            texture_temp: str(""),
            sound: str("default"),
            sound_temp: str(""),
            audio: Audio::new(),
            db: None,
            init: false,
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
            "startup",
            &format!("{}Cursed-Pong/audio/default/startup.ogg", home()),
        );
        bord.audio.add(
            "rickroll",
            &format!("{}Cursed-Pong/audio/default/rickroll.ogg", home()),
        );

        bord.audio.play("startup");

        (bord, window::change_mode(window::Mode::Fullscreen))
    }
    fn title(&self) -> String {
        "Cursed Pong".to_string()
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<GuiMessage> {
        self.updatefn(message);
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
            button("Exit").on_press(GuiMessage::Exit),
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
            button("Exit").on_press(GuiMessage::Exit),
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        let rickroll = container(column![Image::new(format!(
            "{}/Cursed-Pong/bluescreen.png",
            home()
        )),])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        let settings = container(column![
            button("Main Page").on_press(GuiMessage::Change(Page::Main)),
            text(format!("Delay: {}", self.delay)),
            slider(1..=8, self.delay as u8, GuiMessage::Delay),
            text(format!("Current Texture Pack: {}", self.texture)),
            row![
                text("Choose Texture Pack: "),
                text_input("", &self.texture_temp).on_input(GuiMessage::Texture),
                button("Go").on_press(GuiMessage::TextureAssign)
            ],
            text(format!("Current Sound Pack: {}", self.sound)),
            row![
                text("Choose Sound Pack: "),
                text_input("", &self.sound_temp).on_input(GuiMessage::Sound),
                button("Go").on_press(GuiMessage::SoundAssign)
            ],
            button("Check for Updates").on_press(GuiMessage::CheckUpdates),
            button("Exit").on_press(GuiMessage::Exit),
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

        let size = if self.size == 50 { 50 } else { 11 };

        let err = container(column![
            text(&self.err).size(size),
            button("Back to main page").on_press(GuiMessage::Restart),
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        let wait = container(column![text(
            "Please wait while Cursed Pong is being initialized"
        )
        .size(size),])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        let updates = container(column![
            text("Updates Available").size(50),
            text("Launch the Updater to Update Cursed Pong"),
            button("Launch Updater").on_press(GuiMessage::LaunchUpdater),
            button("Back to Main Page").on_press(GuiMessage::Restart),
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        let no_updates = container(column![
            text("No Updates Available").size(50),
            button("Back to Main Page").on_press(GuiMessage::Restart)
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
            Page::Err => err,
            Page::Wait => wait,
            Page::Updates => updates,
            Page::NoUpdates => no_updates,
        }
    }
    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}
