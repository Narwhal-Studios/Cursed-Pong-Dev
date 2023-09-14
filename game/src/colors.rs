use crate::defs::{home, str, F32};
use iced::{
    theme::{self, Theme as itheme},
    Color as icolor,
};
use std::fs;
use toml::{Table, Value};

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}

impl Color {
    pub fn to_irgb(&self, pack: &str) -> (f64, f64, f64) {
        let conf = fs::read_to_string(&format!("{}Cursed-Pong/images/{}/Pack.toml", home(), pack))
            .unwrap()
            .parse::<Table>()
            .unwrap();
        if conf.get("default").unwrap().is_bool() && conf.get("default").unwrap().as_bool().unwrap()
        {
            match self {
                Self::White => (1.0, 1.0, 1.0),
                Self::Black => (0.0, 0.0, 0.0),
                Self::Red => (1.0, 0.0, 0.0),
                Self::Orange => (1.0, 0.5, 0.0),
                Self::Yellow => (1.0, 1.0, 0.0),
                Self::Green => (0.0, 1.0, 0.0),
                Self::Blue => (0.0, 0.0, 1.0),
                Self::Purple => (0.7, 0.0, 0.7),
                Self::Pink => (1.0, 0.5, 0.8),
            }
        } else {
            match conf.get(&self.to_string()).unwrap() {
                Value::Array(array) => (
                    array.get(0).unwrap().as_float().unwrap(),
                    array.get(1).unwrap().as_float().unwrap(),
                    array.get(2).unwrap().as_float().unwrap(),
                ),
                _ => panic!("Not an array"),
            }
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::White => str("white"),
            Self::Black => str("black"),
            Self::Red => str("red"),
            Self::Orange => str("orange"),
            Self::Yellow => str("yellow"),
            Self::Green => str("green"),
            Self::Blue => str("blue"),
            Self::Purple => str("purple"),
            Self::Pink => str("pink"),
        }
    }
}

pub struct Theme {
    background: Color,
    on: Color,
}

impl Theme {
    pub fn new(background: Color, on: Color) -> Self {
        Self {
            background: background,
            on,
        }
    }
    pub fn to_theme(&self, pack: &str) -> itheme {
        let (br, bg, bb) = self.background.to_irgb(pack);
        let (tr, tg, tb) = self.on.to_irgb(pack);

        itheme::custom(theme::Palette {
            background: icolor::from_rgb(br.f32(), bg.f32(), bb.f32()),
            text: icolor::from_rgb(tr.f32(), tg.f32(), tb.f32()),
            primary: icolor::from_rgb(tr.f32(), tg.f32(), tb.f32()),
            success: icolor::from_rgb(0.0, 1.0, 0.0),
            danger: icolor::from_rgb(1.0, 0.0, 0.0),
        })
    }
}
