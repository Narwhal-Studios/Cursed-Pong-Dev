pub mod colors;
pub mod gui;
pub mod gui_parts;

use iced::{Application, Settings};

use gui::Gui;

fn main() -> iced::Result {
    Gui::run(Settings::default())
}
