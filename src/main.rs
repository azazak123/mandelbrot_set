use iced::{Application, Settings};

mod mandelbrot_calculation;
mod ui;

use ui::viewer::Viewer;

pub fn main() -> iced::Result {
    Viewer::run(Settings::default())
}