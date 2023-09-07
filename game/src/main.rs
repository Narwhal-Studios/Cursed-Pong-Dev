pub mod colors;
pub mod defs;
pub mod gui;
pub mod gui_parts;
pub mod updatefn;

use iced::{Application, Settings};

use gui::Gui;

fn main() -> iced::Result {
    Gui::run(Settings::default())
}
