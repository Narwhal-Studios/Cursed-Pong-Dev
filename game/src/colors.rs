use iced::{
    theme::{self, Theme as itheme},
    Color as icolor,
};
use crate::defs::str;

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
    pub fn to_irgb(&self) -> (f32, f32, f32) {
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
    pub fn to_theme(&self) -> itheme {
        let (br, bg, bb) = self.background.to_irgb();
        let (tr, tg, tb) = self.on.to_irgb();

        itheme::custom(theme::Palette {
            background: icolor::from_rgb(br, bg, bb),
            text: icolor::from_rgb(tr, tg, tb),
            primary: icolor::from_rgb(tr, tg, tb),
            success: icolor::from_rgb(0.0, 1.0, 0.0),
            danger: icolor::from_rgb(1.0, 0.0, 0.0),
        })
    }
}
