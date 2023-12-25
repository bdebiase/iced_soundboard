mod styling;

use std::collections::BTreeMap;

use iced::{font, window, Application, Command, Font, Length, Settings, Subscription};

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::{
        static_sound::{StaticSoundData, StaticSoundSettings},
        streaming::{StreamingSoundData, StreamingSoundHandle, StreamingSoundSettings},
        FromFileError, PlaybackRate, PlaybackState,
    },
    tween::Tween,
    Volume,
};
use serde::{Deserialize, Serialize};

const FONT_BYTES_REGULAR: &[u8] = include_bytes!("../fonts/Roboto/Roboto-Regular.ttf");
const FONT_BYTES_BOLD: &[u8] = include_bytes!("../fonts/Roboto/Roboto-Bold.ttf");
const ICONS_FONT_BYTES: &[u8] = include_bytes!("../fonts/icons.ttf");

const FONT_NAME: &'static str = "Roboto";
const TOOLBAR_FONT_SIZE: u16 = 14;
const CONTENT_FONT_SIZE: u16 = 16;
const FOOTER_FONT_SIZE: u16 = 12;
const LOADING_INDICATOR_SIZE: f32 = 120.0;
const LOADING_INDICATOR_SPEED_MS: u64 = 100;

const TITLE: &'static str = "Soundboard";

const SPACING_SMALL: u16 = 5;
const SPACING_NORMAL: u16 = 10;
const SPACING_LARGE: u16 = 20;

fn main() -> iced::Result {
    let (client, _status) =
        jack::Client::new("rust_jack", jack::ClientOptions::NO_START_SERVER).unwrap();
    let out_port = client
        .register_port("rust_out", jack::AudioOut::default())
        .unwrap();

    let active_client = client.activate_async((), ()).unwrap();

    let ports = active_client
        .as_client()
        .ports(None, None, jack::PortFlags::empty());

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
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
enum AudioCommand {
    Play,
    Pause,
    Stop,
    Seek(f64),
}

struct AudioPlayback {
    sound_handle: StreamingSoundHandle<FromFileError>,
    duration: f64,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    SetDirty,
    FontLoaded(Result<(), font::Error>),
    SelectDirectory,
    DirectorySelected(Option<std::path::PathBuf>),
    VolumeChanged(f64),
    SpeedChanged(f64),
    StartPlayback(std::path::PathBuf),
    AudioEvent(usize, AudioCommand),
    UpdatePlaybacks,
}

enum SoundboardApp {
    Loading,
    Loaded(AppState),
}

struct AppState {
    directory: Option<std::path::PathBuf>,
    files: Vec<std::path::PathBuf>,
    audio_manager: Option<AudioManager>,
    active_playbacks: BTreeMap<usize, AudioPlayback>,
    next_id: usize,
    global_volume: f64,
    global_speed: f64,
    saving: bool,
    dirty: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            directory: Default::default(),
            files: Default::default(),
            audio_manager: Default::default(),
            active_playbacks: Default::default(),
            next_id: 0,
            global_volume: 1.0,
            global_speed: 1.0,
            saving: false,
            dirty: false,
        }
    }
}

impl SoundboardApp {
    fn font(&self) -> iced::Font {
        iced::Font {
            weight: iced::font::Weight::Normal,
            family: iced::font::Family::Name(FONT_NAME),
            monospaced: true,
            stretch: iced::font::Stretch::Normal,
        }
    }

    fn bold_font(&self) -> iced::Font {
        iced::Font {
            weight: iced::font::Weight::Bold,
            ..self.font()
        }
    }

    fn view_loading(&self) -> iced::Element<Message> {
        iced::widget::container(
            iced::widget::text("Loading...")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .size(50),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
    }

    fn view_toolbar(&self) -> iced::Element<Message> {
        let mut row = iced::widget::Row::new();

        let text = iced::widget::text("Open".to_uppercase())
            .font(self.bold_font())
            .size(TOOLBAR_FONT_SIZE);
        let button = iced::widget::button(text)
            .on_press(Message::SelectDirectory)
            .style(styling::ToolbarButton::default().into());
        row = row.push(button);

        row = row
            .width(iced::Length::Fill)
            .align_items(iced::Alignment::Center)
            .spacing(SPACING_NORMAL)
            .height(iced::Length::Shrink);

        iced::widget::container(row)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .style(styling::CustomContainer::toolbar().move_to_style())
            .padding([SPACING_NORMAL + SPACING_SMALL, SPACING_LARGE])
            .into()
    }

    fn view_content<'a>(&'a self, state: &'a AppState) -> iced::Element<Message> {
        let volume_slider =
            iced::widget::slider(0.0..=1.0, state.global_volume, Message::VolumeChanged)
                .step(0.001)
                .on_release(Message::SetDirty);
        let speed_slider =
            iced::widget::slider(0.0..=2.0, state.global_speed, Message::SpeedChanged)
                .step(0.001)
                .on_release(Message::SetDirty);
        let buttons_column: iced::Element<Message> = iced::widget::column(
            state
                .files
                .iter()
                .map(|path| {
                    let text = iced::widget::text(path.file_name().unwrap().to_str().unwrap());
                    iced::widget::button(text)
                        .width(iced::Length::Fill)
                        .on_press(Message::StartPlayback(path.clone()))
                        .into()
                })
                .collect(),
        )
        .spacing(10)
        .into();

        let buttons_scrollable = iced::widget::scrollable(buttons_column)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);

        let controls_column: iced::Element<Message> = iced::widget::column(
            state
                .active_playbacks
                .iter()
                .map(|(id, playback)| {
                    let play_button = iced::widget::button("Play")
                        .on_press(Message::AudioEvent(*id, AudioCommand::Play));
                    let pause_button = iced::widget::button("Pause")
                        .on_press(Message::AudioEvent(*id, AudioCommand::Pause));
                    let control_button = if playback.sound_handle.state() == PlaybackState::Playing
                    {
                        pause_button
                    } else {
                        play_button
                    };

                    let slider = iced::widget::slider(
                        0.0..=playback.duration,
                        playback.sound_handle.position(),
                        |value| Message::AudioEvent(*id, AudioCommand::Seek(value)),
                    )
                    .step(0.001);

                    let text = iced::widget::text(format!(
                        "{:.2}:{:.2}",
                        playback.sound_handle.position(),
                        playback.duration,
                    ));

                    let stop_button = iced::widget::button("Stop")
                        .on_press(Message::AudioEvent(*id, AudioCommand::Stop));

                    let row =
                        iced::widget::row!(control_button, slider, text, stop_button).spacing(10);
                    iced::widget::container(row).into()
                })
                .collect(),
        )
        .spacing(10)
        .into();
        let controls_scrollable =
            iced::widget::scrollable(controls_column).width(iced::Length::Fill);

        let main_column = iced::widget::column!(
            volume_slider,
            speed_slider,
            buttons_scrollable,
            controls_scrollable
        )
        .spacing(10);

        iced::widget::container(main_column)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding([SPACING_NORMAL, SPACING_LARGE])
            .center_x()
            .center_y()
            .into()
    }

    fn view_footer(&self) -> iced::Element<'_, Message> {
        let status_message = "Status message";

        let text = iced::widget::text(status_message)
            .font(self.font())
            .size(FOOTER_FONT_SIZE);

        let mut row = iced::widget::Row::new()
            .width(iced::Length::Fill)
            .spacing(SPACING_SMALL)
            .align_items(iced::Alignment::Center);

        row = row.push(text);

        iced::widget::container(row)
            .padding([SPACING_SMALL, SPACING_LARGE])
            .height(iced::Length::Shrink)
            .width(iced::Length::Fill)
            .into()
    }

    fn view_full<'a>(&'a self, state: &'a AppState) -> iced::Element<Message> {
        let toolbar = self.view_toolbar();
        let footer = self.view_footer();
        // let divider_toolbar =
        //     iced::widget::horizontal_rule(0).style(styling::CustomRule::dark().move_to_style());

        let content = self.view_content(state);

        let column = iced::widget::column!(toolbar, content, footer)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_items(iced::Alignment::Center)
            .spacing(0);

        iced::widget::container(column)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

impl Application for SoundboardApp {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = iced::theme::Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut commands: Vec<iced::Command<Message>> = vec![FONT_BYTES_REGULAR, FONT_BYTES_BOLD]
            .iter()
            .map(|&bytes| iced::font::load(std::borrow::Cow::from(bytes)).map(Message::FontLoaded))
            .collect();
        commands.push(iced::Command::perform(SavedState::load(), Message::Loaded));

        (SoundboardApp::Loading, iced::Command::batch(commands))
    }

    fn title(&self) -> String {
        TITLE.into()
    }

    fn theme(&self) -> Self::Theme {
        styling::CustomTheme::new().to_theme()
    }

    fn view(&self) -> iced::Element<Message> {
        match self {
            SoundboardApp::Loading => self.view_loading(),
            SoundboardApp::Loaded(state) => self.view_full(state),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            SoundboardApp::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        let audio_manager =
                            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                                .unwrap();
                        let files = if let Some(dir) = state.directory.clone() {
                            get_files_from_dir(dir)
                        } else {
                            vec![]
                        };

                        *self = SoundboardApp::Loaded(AppState {
                            directory: state.directory,
                            files,
                            audio_manager: Some(audio_manager),
                            global_speed: state.global_speed,
                            global_volume: state.global_volume,
                            ..Default::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        let audio_manager =
                            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                                .unwrap();
                        *self = SoundboardApp::Loaded(AppState {
                            audio_manager: Some(audio_manager),
                            ..Default::default()
                        });
                    }
                    _ => {}
                }
                iced::Command::none()
            }
            SoundboardApp::Loaded(state) => {
                let command = match message {
                    Message::Saved(_) => {
                        println!("Saved!");
                        state.dirty = false;
                        state.saving = false;
                        iced::Command::none()
                    }
                    Message::SetDirty => {
                        println!("Saving changes!");
                        state.dirty = true;
                        iced::Command::none()
                    }
                    Message::SelectDirectory => {
                        println!("Presenting directory file picker...");
                        iced::Command::perform(
                            async {
                                let folder = rfd::AsyncFileDialog::new().pick_folder().await;
                                folder.map(|handle| handle.path().to_path_buf())
                            },
                            Message::DirectorySelected,
                        )
                    }
                    Message::DirectorySelected(path) => {
                        println!("Directory selected: {:?}", path);
                        if let Some(path) = path {
                            state.directory = Some(path.clone());
                            state.files = get_files_from_dir(path);

                            iced::Command::perform(async { Message::SetDirty }, |message| message)
                        } else {
                            iced::Command::none()
                        }
                    }
                    Message::VolumeChanged(value) => {
                        state.global_volume = value;
                        for (_, playback) in state.active_playbacks.iter_mut() {
                            let _ = playback
                                .sound_handle
                                .set_volume(Volume::Amplitude(value), Tween::default());
                        }
                        iced::Command::none()
                    }
                    Message::SpeedChanged(value) => {
                        state.global_speed = value;
                        for (_, playback) in state.active_playbacks.iter_mut() {
                            let _ = playback
                                .sound_handle
                                .set_playback_rate(PlaybackRate::Factor(value), Tween::default());
                        }
                        iced::Command::none()
                    }
                    Message::AudioEvent(id, command) => match command {
                        AudioCommand::Play => {
                            let playback = state.active_playbacks.get_mut(&id).unwrap();
                            let _ = playback.sound_handle.resume(Tween::default());
                            iced::Command::none()
                        }
                        AudioCommand::Pause => {
                            let playback = state.active_playbacks.get_mut(&id).unwrap();
                            let _ = playback.sound_handle.pause(Tween::default());
                            iced::Command::none()
                        }
                        AudioCommand::Stop => {
                            let playback = state.active_playbacks.get_mut(&id).unwrap();
                            let _ = playback.sound_handle.stop(Tween::default());
                            iced::Command::none()
                        }
                        AudioCommand::Seek(position) => {
                            let playback = state.active_playbacks.get_mut(&id).unwrap();
                            let _ = playback.sound_handle.seek_to(position);
                            iced::Command::none()
                        }
                    },
                    Message::UpdatePlaybacks => {
                        state.active_playbacks.retain(|_id, playback| {
                            if playback.sound_handle.state() == PlaybackState::Stopped {
                                false
                            } else {
                                true
                            }
                        });
                        iced::Command::none()
                    }
                    Message::StartPlayback(path) => {
                        let streaming_sound_data = StreamingSoundData::from_file(
                            path.clone(),
                            StreamingSoundSettings::default()
                                .volume(Volume::Amplitude(state.global_volume))
                                .playback_rate(state.global_speed),
                        )
                        .unwrap();
                        let sound_handle = state
                            .audio_manager
                            .as_mut()
                            .unwrap()
                            .play(streaming_sound_data)
                            .unwrap();

                        // get duration from static sound (doesn't work with streaming)
                        let static_sound_data =
                            StaticSoundData::from_file(path, StaticSoundSettings::default())
                                .unwrap();
                        let duration = static_sound_data.duration().as_secs_f64();

                        let playback = AudioPlayback {
                            sound_handle,
                            duration,
                        };

                        // let id = self.audio_manager.as_ref().unwrap().num_sounds();
                        state.active_playbacks.insert(state.next_id, playback);
                        state.next_id += 1;

                        iced::Command::none()
                    }
                    _ => iced::Command::none(),
                };

                let save = if state.dirty && !state.saving {
                    // state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            directory: state.directory.clone(),
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
        let update_timer = iced::time::every(std::time::Duration::from_millis(100))
            .map(|_| Message::UpdatePlaybacks);

        iced::Subscription::from(update_timer)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    directory: Option<std::path::PathBuf>,
    global_volume: f64,
    global_speed: f64,
}

#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
enum SaveError {
    File,
    Write,
    Format,
}

impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("rs", "Iced", "Soundboard")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or_default()
        };

        path.push("config.json");

        path
    }

    async fn load() -> Result<SavedState, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::File)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::File)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        // async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}

fn get_files_from_dir(directory: std::path::PathBuf) -> Vec<std::path::PathBuf> {
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(directory)
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
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    files
}
