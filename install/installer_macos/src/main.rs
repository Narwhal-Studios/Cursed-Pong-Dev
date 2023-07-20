pub mod gui;

use gui::Gui;
use iced::{Application, Result, Settings};

fn main() -> Result {
    Gui::run(Settings::default())
}
