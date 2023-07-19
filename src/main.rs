pub mod colors;
pub mod gui;
pub mod gui_parts;

use iced::{window, Application, Settings};

use gui::Gui;

fn main() -> iced::Result {
    Gui::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Default,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
