use iced::{
    theme::{self, Theme as itheme},
    Color as icolor,
};
use raster::Color as RColor;

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
    pub fn to_rcolor(&self) -> RColor {
        match self {
            Self::White => RColor::rgb(255, 255, 255),
            Self::Black => RColor::rgb(0, 0, 0),
            Self::Red => RColor::rgb(255, 0, 0),
            Self::Orange => RColor::rgb(255, 127, 0),
            Self::Yellow => RColor::rgb(255, 255, 0),
            Self::Green => RColor::rgb(0, 255, 0),
            Self::Blue => RColor::rgb(0, 0, 255),
            Self::Purple => RColor::rgb(178, 0, 178),
            Self::Pink => RColor::rgb(255, 127, 204),
        }
    }
}

pub struct Theme {
    background: Color,
    on: Color,
    off: Color,
}

impl Theme {
    pub fn new(background: Color, on: Color, off: Color) -> Self {
        Self {
            background,
            on,
            off,
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
    pub fn to_rcolor(&self) -> (RColor, RColor) {
        (self.on.to_rcolor(), self.off.to_rcolor())
    }
}
