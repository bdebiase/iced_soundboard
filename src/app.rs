use crate::{
    audio::{AudioClip, AudioCommand, AudioPlayback},
    saving::{LoadError, SaveError, SavedState},
    style::{self, FONT_BYTES_BOLD, FONT_BYTES_REGULAR},
};

use iced::{executor, font, theme, time, Application, Command, Element, Subscription};
use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::{streaming::StreamingSoundData, PlaybackRate, PlaybackState},
    tween::Tween,
    Volume,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use youtube_dl::YoutubeDl;

const TITLE: &'static str = "Soundboard";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub name: String,
    pub directory: std::path::PathBuf,

    #[serde(skip)]
    pub clips: Vec<AudioClip>,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(Result<(), font::Error>),
    Saved(Result<(), SaveError>),
    Loaded(Result<SavedState, LoadError>),

    SelectTab(usize),
    CloseTab(usize),
    NewTab,
    CreateTab(Option<std::path::PathBuf>),
    RefreshClips,

    SetDirty,
    VolumeToggled,
    VolumeChanged(f64),
    SpeedToggled,
    SpeedChanged(f64),

    AudioEvent(usize, AudioCommand),
    StartPlayback(AudioClip),
    UpdatePlaybacks,
    StopAllPlaybacks,

    ToggleDownloadPopup,
    DownloadUrlChanged(String),
    StartDownload,
    UpdateDownloadProgress(f32),
    DownloadFinished(Result<(), ()>),
}

pub enum SoundboardApp {
    Loading,
    Loaded(AppState),
}

pub struct AppState {
    pub tabs: Vec<Tab>,
    pub current_tab: usize,

    pub audio_manager: Option<AudioManager>,
    pub active_playbacks: BTreeMap<usize, AudioPlayback>,
    pub next_id: usize,

    pub volume_enabled: bool,
    pub global_volume: f64,
    pub global_speed: f64,
    pub speed_enabled: bool,

    pub saving: bool,
    pub dirty: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            tabs: vec![],
            current_tab: 0,
            audio_manager: Default::default(),
            active_playbacks: Default::default(),
            next_id: 0,
            volume_enabled: true,
            global_volume: 1.0,
            global_speed: 1.0,
            speed_enabled: true,
            saving: false,
            dirty: false,
        }
    }
}

impl AppState {
    pub fn save(&mut self) {
        self.dirty = false;
        self.saving = false;
    }

    pub fn toggle_global_volume(&mut self) {
        self.volume_enabled = !self.volume_enabled;
        self.update_playbacks_volume();
    }

    pub fn toggle_global_speed(&mut self) {
        self.speed_enabled = !self.speed_enabled;
        self.update_playbacks_speed();
    }

    pub fn set_global_volume(&mut self, value: f64) {
        self.global_volume = value;
        self.update_playbacks_volume();
    }

    pub fn get_global_volume(&self) -> f64 {
        if self.volume_enabled {
            self.global_volume
        } else {
            0.0
        }
    }

    pub fn set_global_speed(&mut self, value: f64) {
        self.global_speed = value;
        self.update_playbacks_speed();
    }

    pub fn start_download(&mut self) -> Result<(), ()> {
        if let Some(tab) = self.get_current_tab() {
            println!("Starting download...");

            let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
                .youtube_dl_path(tab.directory.clone())
                .run()
                .unwrap();
            let title = output.into_single_video().unwrap().title;
            println!("Video title: {title:?}");
        } else {
            println!("No tab selected.");
        }
        Ok(())
    }

    pub fn refresh_clips(&mut self) {
        if let Some(tab) = self.tabs.get_mut(self.current_tab) {
            tab.clips = load_audio_clips(tab.directory.clone());
            println!("Clips refreshed.");
        } else {
            println!("No clips to refresh.");
        }
    }

    fn update_playbacks_volume(&mut self) {
        let volume = self.get_global_volume();
        for (_, playback) in self.active_playbacks.iter_mut() {
            let _ = playback
                .handle
                .set_volume(Volume::Amplitude(volume), Tween::default());
        }
    }

    fn update_playbacks_speed(&mut self) {
        let speed = self.get_global_speed();
        for (_, playback) in self.active_playbacks.iter_mut() {
            let _ = playback
                .handle
                .set_playback_rate(PlaybackRate::Factor(speed), Tween::default());
        }
    }

    pub fn update_playbacks(&mut self) {
        self.active_playbacks.retain(|_id, playback| {
            if playback.handle.state() == PlaybackState::Stopped {
                false
            } else {
                true
            }
        });
    }

    pub fn stop_all_playbacks(&mut self) {
        for (_, playback) in self.active_playbacks.iter_mut() {
            let _ = playback.handle.stop(Tween::default());
        }
    }

    pub fn start_playback(&mut self, clip: AudioClip) {
        let sound_data = StreamingSoundData::from_file(
            clip.clone().path,
            //StreamingSoundSettings::default()
            //    .volume(Volume::Amplitude(self.get_global_volume()))
            //    .playback_rate(self.get_global_speed()),
        )
        .unwrap();

        let mut sound_handle = self
            .audio_manager
            .as_mut()
            .unwrap()
            .play(sound_data)
            .unwrap();

        sound_handle.set_playback_rate(self.get_global_speed(), Tween::default());
        sound_handle.set_volume(
            Volume::Amplitude(self.get_global_volume()),
            Tween::default(),
        );

        let playback = AudioPlayback {
            clip,
            handle: sound_handle,
        };

        self.active_playbacks.insert(self.next_id, playback);
        self.next_id += 1;
    }

    pub fn add_tab(&mut self, tab: Tab) {
        self.tabs.push(tab);
        self.current_tab = self.tabs.len() - 1;
    }

    pub fn close_tab(&mut self, index: usize) {
        self.tabs.remove(index);
        self.current_tab = if self.tabs.is_empty() {
            0
        } else {
            usize::max(0, usize::min(self.current_tab, self.tabs.len() - 1))
        };
    }

    pub fn select_tab(&mut self, index: usize) {
        self.current_tab = index;
    }

    pub fn get_current_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.current_tab)
    }

    pub fn get_global_speed(&self) -> f64 {
        if self.speed_enabled {
            self.global_speed
        } else {
            1.0
        }
    }

    pub fn set_dirty(&self) -> bool {
        self.dirty
    }
}

impl Application for SoundboardApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = theme::Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut commands: Vec<Command<Message>> = vec![FONT_BYTES_REGULAR, FONT_BYTES_BOLD]
            .iter()
            .map(|&bytes| font::load(std::borrow::Cow::from(bytes)).map(Message::FontLoaded))
            .collect();

        commands.push(Command::perform(SavedState::load(), Message::Loaded));

        (SoundboardApp::Loading, Command::batch(commands))
    }

    fn title(&self) -> String {
        TITLE.into()
    }

    fn theme(&self) -> Self::Theme {
        style::theme::CustomTheme::default().into()
    }

    fn view(&self) -> Element<Message> {
        match self {
            SoundboardApp::Loading => self.view_loading(),
            SoundboardApp::Loaded(_) => self.view_full(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        // TODO: move
        // handle loading
        match self {
            SoundboardApp::Loading => match message {
                // if loaded with saved state, set state
                Message::Loaded(Ok(state)) => {
                    let audio_manager =
                        AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                            .unwrap();

                    let mut app_state = AppState {
                        tabs: state.tabs.clone(),
                        current_tab: state.current_tab,
                        audio_manager: Some(audio_manager),
                        global_speed: state.global_speed,
                        global_volume: state.global_volume,
                        ..Default::default()
                    };
                    app_state.refresh_clips(); // TODO: move to async

                    *self = SoundboardApp::Loaded(app_state);

                    Command::none()
                }
                // if loaded with error or no state, set default state
                Message::Loaded(Err(_)) => {
                    let audio_manager =
                        AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                            .unwrap();

                    *self = SoundboardApp::Loaded(AppState {
                        audio_manager: Some(audio_manager),
                        ..Default::default()
                    });

                    Command::none()
                }
                _ => Command::none(),
            },
            SoundboardApp::Loaded(state) => Command::batch(vec![
                crate::ui::update(state, &message),
                crate::audio::update(state, &message),
            ]),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let update_timer =
            time::every(std::time::Duration::from_millis(100)).map(|_| Message::UpdatePlaybacks);

        Subscription::from(update_timer)
    }
}

fn load_audio_clips(path: std::path::PathBuf) -> Vec<AudioClip> {
    let mut clips = vec![];
    let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().unwrap().is_file())
        .filter(|entry| {
            if let Some(ext) = entry.path().extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                ext == "mp3" || ext == "wav" || ext == "flac" || ext == "ogg"
            } else {
                false
            }
        })
        .map(|entry| entry.path())
        .collect();
    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for path in paths {
        let name = path.file_name().unwrap().to_str().unwrap().to_owned();
        let duration = get_audio_duration(&path);

        clips.push(AudioClip {
            name,
            path,
            duration,
        });
    }

    clips
}

fn get_audio_duration(path: &std::path::PathBuf) -> std::time::Duration {
    use lofty::prelude::AudioFile;

    let tagged_file = lofty::probe::Probe::open(path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    let properties = tagged_file.properties();
    let duration = properties.duration();

    duration
}
