use crate::{
    audio::{AudioClip, AudioCommand, AudioPlayback},
    saving::{LoadError, SaveError, SavedState},
    style::{self, FONT_BYTES_BOLD, FONT_BYTES_REGULAR},
};

use iced::{executor, font, theme, time, Application, Command, Element, Subscription};
use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::{
        streaming::{StreamingSoundData, StreamingSoundSettings},
        PlaybackRate, PlaybackState,
    },
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
    tabs: Vec<Tab>,
    current_tab: usize,

    audio_manager: Option<AudioManager>,
    active_playbacks: BTreeMap<usize, AudioPlayback>,
    next_id: usize,

    volume_enabled: bool,
    global_volume: f64,
    global_speed: f64,
    speed_enabled: bool,

    show_download_popup: bool,
    download_url: String,
    download_progress: f32,

    saving: bool,
    dirty: bool,
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
            show_download_popup: false,
            download_url: Default::default(),
            download_progress: 0.0,
            saving: false,
            dirty: false,
        }
    }
}

impl AppState {
    fn toggle_global_volume(&mut self) {
        self.volume_enabled = !self.volume_enabled;
        self.update_playbacks_volume();
    }

    fn toggle_global_speed(&mut self) {
        self.speed_enabled = !self.speed_enabled;
        self.update_playbacks_speed();
    }

    fn toggle_download_popup(&mut self) {
        self.show_download_popup = !self.show_download_popup;
    }

    fn set_global_volume(&mut self, value: f64) {
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

    fn set_global_speed(&mut self, value: f64) {
        self.global_speed = value;
        self.update_playbacks_speed();
    }

    fn set_download_url(&mut self, value: String) {
        self.download_url = value;
    }

    fn start_download(&mut self) -> Result<(), ()> {
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

    fn refresh_clips(&mut self) {
        if let Some(tab) = self.tabs.get_mut(self.current_tab) {
            tab.clips = load_audio_clips(tab.directory.clone());
            println!("Clips refreshed.");
        } else {
            println!("No clips to refresh.");
        }
    }

    fn set_dirty(&mut self) {
        self.dirty = true;
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

    fn update_playbacks(&mut self) {
        self.active_playbacks.retain(|_id, playback| {
            if playback.handle.state() == PlaybackState::Stopped {
                false
            } else {
                true
            }
        });
    }

    fn stop_all_playbacks(&mut self) {
        for (_, playback) in self.active_playbacks.iter_mut() {
            let _ = playback.handle.stop(Tween::default());
        }
    }

    fn start_playback(&mut self, clip: AudioClip) {
        let sound_data = StreamingSoundData::from_file(
            clip.clone().path,
            StreamingSoundSettings::default()
                .volume(Volume::Amplitude(self.get_global_volume()))
                .playback_rate(self.get_global_speed()),
        )
        .unwrap();

        let sound_handle = self
            .audio_manager
            .as_mut()
            .unwrap()
            .play(sound_data)
            .unwrap();

        let playback = AudioPlayback {
            clip,
            handle: sound_handle,
        };

        self.active_playbacks.insert(self.next_id, playback);
        self.next_id += 1;
    }

    fn add_tab(&mut self, tab: Tab) {
        self.tabs.push(tab);
        self.current_tab = self.tabs.len() - 1;
    }

    fn close_tab(&mut self, index: usize) {
        self.tabs.remove(index);
        self.current_tab = if self.tabs.is_empty() {
            0
        } else {
            usize::max(0, usize::min(self.current_tab, self.tabs.len() - 1))
        };
    }

    fn select_tab(&mut self, index: usize) {
        self.current_tab = index;
    }

    pub fn get_tabs(&self) -> &Vec<Tab> {
        &self.tabs
    }

    pub fn get_current_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.current_tab)
    }

    pub fn get_active_playbacks(&self) -> &BTreeMap<usize, AudioPlayback> {
        &self.active_playbacks
    }

    pub fn get_global_speed(&self) -> f64 {
        if self.speed_enabled {
            self.global_speed
        } else {
            1.0
        }
    }

    pub fn get_download_url(&self) -> &str {
        &self.download_url
    }

    pub fn get_download_progress(&self) -> f32 {
        self.download_progress
    }

    pub fn global_volume_enabled(&self) -> bool {
        self.volume_enabled
    }

    pub fn global_speed_enabled(&self) -> bool {
        self.speed_enabled
    }

    pub fn get_show_download_popup(&self) -> bool {
        self.show_download_popup
    }

    pub fn get_current_tab_index(&self) -> usize {
        self.current_tab
    }
}

impl Application for SoundboardApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = theme::Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut commands: Vec<Command<Message>> = vec![
            FONT_BYTES_REGULAR,
            FONT_BYTES_BOLD,
            iced_aw::graphics::icons::ICON_FONT_BYTES,
        ]
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
            SoundboardApp::Loaded(state) => {
                let command = match message {
                    Message::Saved(_) => {
                        println!("Saved!");
                        state.dirty = false;
                        state.saving = false;

                        Command::none()
                    }
                    Message::SelectTab(index) => {
                        println!("Tab selected: {}", index);

                        state.select_tab(index);
                        state.set_dirty();

                        if state.get_current_tab().unwrap().clips.is_empty() {
                            println!("Tab is empty, refreshing clips...");
                            state.refresh_clips(); // TODO: move to async
                        }

                        Command::none()
                    }
                    Message::CloseTab(index) => {
                        println!("Tab closed: {}", index);

                        state.close_tab(index);
                        state.set_dirty();

                        Command::none()
                    }
                    Message::NewTab => {
                        println!("New tab");
                        println!("Presenting directory file picker...");

                        Command::perform(get_dir_async(), Message::CreateTab)
                    }
                    Message::CreateTab(path) => {
                        if let Some(path) = path {
                            println!("Creating new tab with path: {:?}", path);

                            state.add_tab(Tab {
                                name: path.file_name().unwrap().to_str().unwrap().to_owned(),
                                directory: path,
                                clips: vec![],
                            });
                            state.set_dirty();
                            state.refresh_clips(); // TODO: move to async
                        } else {
                            println!("No path provided, tab not created");
                        }

                        Command::none()
                    }
                    Message::RefreshClips => {
                        state.refresh_clips();

                        Command::none()
                    }
                    Message::SetDirty => {
                        state.set_dirty();

                        Command::none()
                    }
                    Message::VolumeToggled => {
                        state.toggle_global_volume();

                        Command::none()
                    }
                    Message::VolumeChanged(value) => {
                        if !state.global_volume_enabled() {
                            state.toggle_global_volume()
                        }
                        state.set_global_volume(value);

                        Command::none()
                    }
                    Message::SpeedToggled => {
                        state.toggle_global_speed();

                        Command::none()
                    }
                    Message::SpeedChanged(value) => {
                        if !state.global_speed_enabled() {
                            state.toggle_global_speed()
                        }
                        state.set_global_speed(value);

                        Command::none()
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
                                    let _ = playback.handle.seek_to(position);
                                }
                            }
                        }

                        Command::none()
                    }
                    Message::StartPlayback(clip) => {
                        state.start_playback(clip);

                        Command::none()
                    }
                    Message::StopAllPlaybacks => {
                        state.stop_all_playbacks();

                        Command::none()
                    }
                    Message::UpdatePlaybacks => {
                        state.update_playbacks();

                        Command::none()
                    }
                    Message::ToggleDownloadPopup => {
                        state.toggle_download_popup();

                        Command::none()
                    }
                    Message::DownloadUrlChanged(value) => {
                        let _ = state.set_download_url(value);

                        Command::none()
                    }
                    Message::StartDownload => {
                        let _ = state.start_download();

                        Command::none()
                        // Command::perform(state.start_download(), Message::DownloadFinished)
                    }
                    Message::UpdateDownloadProgress(_) => Command::none(),
                    Message::DownloadFinished(_) => {
                        println!("Download finished!");

                        Command::none()
                    }
                    _ => Command::none(),
                };

                let save = if state.dirty && !state.saving {
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            tabs: state.tabs.clone(),
                            current_tab: state.current_tab,
                            global_speed: state.global_speed,
                            global_volume: state.global_volume,
                        }
                        .save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                };

                Command::batch(vec![command, save])
            }
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
    use lofty::AudioFile;

    let tagged_file = lofty::Probe::open(path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    let properties = tagged_file.properties();
    let duration = properties.duration();

    duration
}

async fn get_dir_async() -> Option<std::path::PathBuf> {
    let folder = rfd::AsyncFileDialog::new().pick_folder().await;
    let path = folder.map(|handle| handle.path().to_path_buf());

    if let Some(path) = path {
        Some(path)
    } else {
        None
    }
}
