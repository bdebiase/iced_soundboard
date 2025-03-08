use iced::Command;
use kira::{
    sound::{streaming::StreamingSoundHandle, FromFileError},
    Tween,
};
use std::time::Duration;

use crate::app::{AppState, Message};

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

pub fn update(state: &mut AppState, message: &Message) -> Command<Message> {
    match message {
        Message::VolumeToggled => {
            state.toggle_global_volume();
        }
        Message::VolumeChanged(value) => {
            if !state.volume_enabled {
                state.toggle_global_volume();
            }
            state.set_global_volume(*value);
        }
        Message::SpeedToggled => {
            state.toggle_global_speed();
        }
        Message::SpeedChanged(value) => {
            if !state.speed_enabled {
                state.toggle_global_speed();
            }
            state.set_global_speed(*value);
        }
        Message::AudioEvent(id, command) => {
            if let Some(playback) = state.active_playbacks.get_mut(&id) {
                match command {
                    AudioCommand::Play => {
                        let _ = playback.handle.resume(Tween::default());
                    }
                    AudioCommand::Pause => {
                        let _ = playback.handle.pause(Tween::default());
                    }
                    AudioCommand::Stop => {
                        let _ = playback.handle.stop(Tween::default());
                    }
                    AudioCommand::Seek(position) => {
                        let _ = playback.handle.seek_to(*position);
                    }
                }
            }
        }
        Message::StartPlayback(clip) => {
            state.start_playback(clip.to_owned());
        }
        Message::StopAllPlaybacks => {
            state.stop_all_playbacks();
        }
        Message::UpdatePlaybacks => {
            state.update_playbacks();
        }
        _ => {}
    };

    Command::none()
}
