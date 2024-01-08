mod app;
mod audio;
mod saving;
mod style;
mod ui;

use crate::app::SoundboardApp;

use iced::{window, Application, Font, Settings};
use style::{FONT_NAME, FONT_SIZE_DEFAULT};

fn main() -> iced::Result {
    // let (client, _status) =
    //     jack::Client::new("rust_jack", jack::ClientOptions::NO_START_SERVER).unwrap();
    // let out_port = client
    //     .register_port("rust_out", jack::AudioOut::default())
    //     .unwrap();

    // let active_client = client.activate_async((), ()).unwrap();

    // let ports = active_client
    //     .as_client()
    //     .ports(None, None, jack::PortFlags::empty());

    // if let Some(first_port) = ports.get(0) {
    //     active_client
    //         .as_client()
    //         .connect_ports_by_name(&out_port.name().unwrap(), &first_port)
    //         .unwrap();
    // }

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
