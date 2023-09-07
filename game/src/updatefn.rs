use crate::defs::{GuiMessage, Page, home, str};
use crate::gui::Gui;
use crate::gui_parts::{
    GuiParts,
    Velocity,
    Position
};
use crate::colors::{
    Color,
    Theme
};
use std::process;
use rand::Rng;

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
            GuiMessage::Change(page) => {
                match page {
                    Page::Play => {
                        self.is_playing = true;
                        self.page = Page::Play;
                    }
                    _ => {
                        self.is_playing = false;
                        self.page = page;
                    }
                }
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
                    theme: Theme::new(Color::White, Color::Blue).to_theme(),
                    ivalue,
                    onw: str("default/blue.png"),
                    offw: str("default/white.png"),
                    score: 0,
                    err: String::new(),
                    id: 0,
                    size: 50,
                    text: String::new(),
                    will: Position::new(13, 23),
                    rick: Position::new(23, 13),
                    texture: (&self.texture).to_string(),
                    texture_temp: str(""),
                };
                bord.cre_bord();
                bord.draw_bat();
                bord.show_pixel();

                *self = bord;
            }
            GuiMessage::Exit => process::exit(0),
            GuiMessage::Texture(text) => self.texture_temp = text,
            GuiMessage::TextureAssign => self.texture = (&self.texture_temp).to_string(),
        }
    }
}
