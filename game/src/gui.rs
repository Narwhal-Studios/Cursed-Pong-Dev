use crate::colors::{Color, Theme};
use crate::defs::{home, str, sw, GuiMessage, Page};
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
use std::time::Duration;
use version::version;
use crate::updatefn::Fns;

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
            page: Page::Main,
            bat_y: 8,
            err: String::new(),
            position: Position::new(16, 17),
            is_playing: false,
            theme: Theme::new(Color::White, Color::Blue).to_theme(),
            ivalue,
            onw: str("default/blue.png"),
            offw: str("default/white.png"),
            score: 0,
            id: 0,
            size: 50,
            text: String::new(),
            will: Position::new(13, 23),
            rick: Position::new(23, 13),
            texture: str("default"),
            texture_temp: str(""),
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
            row![text("Choose Texture Pack: "), text_input("", &self.texture_temp).on_input(GuiMessage::Texture), button("Go").on_press(GuiMessage::TextureAssign)],
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
            Page::Err => err,
        }
    }
    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}
