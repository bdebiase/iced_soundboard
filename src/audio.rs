use kira::sound::{streaming::StreamingSoundHandle, FromFileError};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum AudioCommand {
    Play,
    Pause,
    Stop,
    Seek(f64),
}

pub struct AudioPlayback {
    pub clip: AudioClip,
    pub handle: StreamingSoundHandle<FromFileError>,
}

#[derive(Debug, Clone)]
pub struct AudioClip {
    pub name: String,
    pub path: std::path::PathBuf,
    pub duration: Duration,
}
