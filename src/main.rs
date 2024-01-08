mod app;
mod audio;
mod saving;
mod style;
mod ui;

use crate::app::SoundboardApp;

use iced::{window, Application, Font, Settings};
use style::{FONT_NAME, FONT_SIZE_DEFAULT};

fn main() -> iced::Result {
    SoundboardApp::run(Settings {
        window: window::Settings {
            size: (500, 800),
            min_size: Some((400, 200)),
            ..Default::default()
        },
        default_font: Font::with_name(FONT_NAME),
        default_text_size: FONT_SIZE_DEFAULT as f32,
        ..Default::default()
    })
}
