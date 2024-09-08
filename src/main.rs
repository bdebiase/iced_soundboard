mod app;
mod audio;
mod saving;
mod style;
mod ui;

use crate::app::SoundboardApp;

use iced::{window, Application, Font, Settings, Size};
use style::{FONT_NAME, FONT_SIZE_DEFAULT};

fn main() -> iced::Result {
    SoundboardApp::run(Settings {
        window: window::Settings {
            size: Size::new(500.0, 800.0),
            min_size: Some(Size::new(400.0, 200.0)),
            ..Default::default()
        },
        default_font: Font::with_name(FONT_NAME),
        default_text_size: FONT_SIZE_DEFAULT.into(),
        ..Default::default()
    })
}
