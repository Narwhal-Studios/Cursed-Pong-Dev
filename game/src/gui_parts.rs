use crate::colors::{Color, Theme};
use crate::defs::{home, sw};
use crate::gui::Gui;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Velocity {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub trait GuiParts {
    fn draw_bat(&mut self);
    fn show_pixel(&mut self);
    fn clear(&mut self);
    fn check_bat(&self) -> bool;
    fn addx(&mut self);
    fn addy(&mut self);
    fn cre_bord(&mut self);
    fn changex(&mut self);
    fn changey(&mut self);
    fn checkx(&self) -> bool;
    fn checky(&self) -> bool;
    fn toggle_theme(&mut self);
    fn will(&mut self);
    fn check_will(&self) -> bool;
    fn add_will(&mut self);
    fn rick(&mut self);
    fn check_rick(&self) -> bool;
    fn add_rick(&mut self);
}

impl GuiParts for Gui {
    fn draw_bat(&mut self) {
        self.value[self.bat_y - 1][1] = true;
        self.value[self.bat_y][1] = true;
        self.value[self.bat_y + 1][1] = true;
        self.ivalue[self.bat_y - 2][0] = format!("{}Cursed-Pong{}{}", home(), sw().1, &self.onw);
        self.ivalue[self.bat_y - 1][0] = format!("{}Cursed-Pong{}{}", home(), sw().1, &self.onw);
        self.ivalue[self.bat_y][0] = format!("{}Cursed-Pong{}{}", home(), sw().1, &self.onw);
    }

    fn clear(&mut self) {
        self.value = [[false; 34]; 34];
        let path = &format!("{}Cursed-Pong/{}", home(), self.offw);
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
        self.add_will();
        self.add_rick();
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
            format!("{}Cursed-Pong/{}", home(), &self.onw);
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
    fn checkx(&self) -> bool {
        if self.position.x == 1 || self.position.x == 32 {
            true
        } else {
            false
        }
    }
    fn checky(&self) -> bool {
        if self.position.y == 1 || self.position.y == 32 {
            true
        } else {
            false
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
        let mut rng = thread_rng();
        let (background, on) = (colors[rng.gen_range(0..=8)], colors[rng.gen_range(0..=8)]);
        self.theme = Theme::new(background, on).to_theme(&self.texture);
        self.onw = format!("images/{}/{}.png", self.texture, on.to_string());
        self.offw = format!("images/{}/{}.png", self.texture, background.to_string());
        self.will();
        self.rick();
        self.audio.play(&background.to_string());
    }
    fn will(&mut self) {
        let mut rand_num = thread_rng();
        self.will = Position::new(rand_num.gen_range(1..=31), rand_num.gen_range(0..=31));
        self.ivalue[self.will.y][self.will.x] =
            format!("{}Cursed-Pong/images/{}/will.png", home(), self.texture);
    }
    fn add_will(&mut self) {
        self.ivalue[self.will.y][self.will.y] =
            format!("{}Cursed-Pong/images/{}/will.png", home(), self.texture);
    }
    fn check_will(&self) -> bool {
        if self.position.x == self.will.x && self.position.y == self.will.y {
            true
        } else {
            false
        }
    }
    fn rick(&mut self) {
        let mut rand_num = thread_rng();
        self.rick = Position::new(rand_num.gen_range(1..=31), rand_num.gen_range(0..=31));
        self.ivalue[self.rick.y][self.rick.x] =
            format!("{}Cursed-Pong/images/{}/rick.png", home(), self.texture);
    }
    fn add_rick(&mut self) {
        self.ivalue[self.rick.y][self.rick.x] =
            format!("{}Cursed-Pong/images/{}/rick.png", home(), self.texture);
    }
    fn check_rick(&self) -> bool {
        if self.position.x == self.rick.x && self.position.y == self.rick.y {
            true
        } else {
            false
        }
    }
}
