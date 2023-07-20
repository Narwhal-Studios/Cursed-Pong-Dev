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
    fn toggle_theme(&mut self);
}
