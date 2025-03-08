use crate::{
    app::{AppState, Message, SoundboardApp, Tab},
    audio::AudioCommand,
    saving::SavedState,
    style::{self, icons, BORDER_RADIUS, FONT_NAME, FONT_SIZE_TABS, SPACING_NORMAL, SPACING_SMALL},
};

use iced::{
    alignment, font,
    widget::{
        self,
        scrollable::{Direction, Properties},
    },
    Alignment, Command, Element, Font, Length,
};
use kira::sound::PlaybackState;

const TOOL_BUTTON_SIZE: Length = Length::Fixed(26.0);
const TOOL_BUTTON_SIZE_SMALL: Length = Length::Fixed(24.0);

#[allow(unused)]
impl SoundboardApp {
    pub fn font(&self) -> Font {
        Font {
            family: font::Family::Name(FONT_NAME),
            ..Default::default()
        }
    }

    pub fn bold_font(&self) -> Font {
        Font {
            weight: font::Weight::Bold,
            ..self.font()
        }
    }

    pub fn view_loading(&self) -> Element<Message> {
        widget::container(
            widget::text("Loading...")
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(50),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
    }

    pub fn view_full(&self) -> Element<Message> {
        match self {
            Self::Loaded(state) => {
                let tab_bar = self.view_tab_bar();
                let content = self.view_content();
                let content_column = widget::column!(tab_bar, content).height(Length::Fill);

                let controls = self.view_controls();
                let playbacks = self.view_playbacks();

                let mut column_widgets = vec![];
                column_widgets.push(content_column.into());
                if !state.active_playbacks.is_empty() {
                    column_widgets.push(playbacks.into());
                }
                column_widgets.push(controls.into());

                let main_column = widget::column(column_widgets).spacing(SPACING_SMALL);

                let underlay: Element<Message> = widget::container(main_column)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(SPACING_NORMAL)
                    .into();

                underlay.into()
            }
            Self::Loading => unreachable!(),
        }
    }

    fn view_tab_bar(&self) -> Element<Message> {
        match self {
            Self::Loaded(state) => {
                let len = state.tabs.len();
                let tabs = state
                    .tabs
                    .iter()
                    .enumerate()
                    .fold(widget::Row::new(), |row, (idx, tab)| {
                        let text = widget::text(tab.name.to_owned()).size(FONT_SIZE_TABS);
                        let close_button = widget::button(icons::cancel())
                            .width(TOOL_BUTTON_SIZE_SMALL)
                            .height(TOOL_BUTTON_SIZE_SMALL)
                            .on_press(Message::CloseTab(idx))
                            .style(style::button::CustomButton::flat());

                        let button = widget::button(
                            widget::row!(text, close_button)
                                .spacing(SPACING_NORMAL)
                                .align_items(Alignment::Center),
                        )
                        .padding([SPACING_SMALL, SPACING_NORMAL])
                        .on_press(Message::SelectTab(idx))
                        .style(
                            style::button::CustomButton::tab(state.current_tab == idx)
                                .with_border_radius([
                                    if idx == 0 { BORDER_RADIUS } else { 0.0 },
                                    if idx == len - 1 { BORDER_RADIUS } else { 0.0 },
                                    0.0,
                                    0.0,
                                ]), // .with_border_radius(if len == 1 {
                                    //     [BORDER_RADIUS, BORDER_RADIUS, 0.0, 0.0]
                                    // } else if idx == 0 {
                                    //     [BORDER_RADIUS, 0.0, 0.0, 0.0]
                                    // } else if idx == len - 1 {
                                    //     [0.0, BORDER_RADIUS, 0.0, 0.0]
                                    // } else {
                                    //     [0.0; 4]
                                    // })
                        );
                        row.push(button)
                    })
                    .width(Length::Shrink);

                let tabs_container = {
                    let scrollable = widget::scrollable(tabs)
                        .direction(Direction::Horizontal(scrollable_properties()))
                        .style(style::scrollable::CustomScrollable::default());

                    let add_button = widget::button(icons::plus())
                        .on_press(Message::NewTab)
                        .width(TOOL_BUTTON_SIZE)
                        .height(TOOL_BUTTON_SIZE)
                        .style(style::button::CustomButton::default());

                    widget::row!(scrollable, add_button)
                        .width(Length::Fill)
                        .spacing(SPACING_NORMAL)
                        .align_items(Alignment::Center)
                };

                let buttons_row = {
                    // let download_button = widget::button(icons::download())
                    //     .width(TOOL_BUTTON_SIZE)
                    //     .height(TOOL_BUTTON_SIZE)
                    //     .style(style::button::CustomButton::default().into());
                    // // .on_press(Message::ToggleDownloadPopup);

                    let refresh_button = widget::button(icons::refresh())
                        .width(TOOL_BUTTON_SIZE)
                        .height(TOOL_BUTTON_SIZE)
                        .on_press(Message::RefreshClips)
                        .style(style::button::CustomButton::default());

                    widget::row!(refresh_button)
                        .spacing(SPACING_SMALL)
                        .align_items(Alignment::Center)
                };

                widget::container(
                    widget::row!(tabs_container, buttons_row)
                        .spacing(SPACING_NORMAL)
                        .align_items(Alignment::Center),
                )
                .into()
            }
            Self::Loading => unreachable!(),
        }
    }

    fn view_controls(&self) -> Element<Message> {
        match self {
            Self::Loaded(state) => {
                let sliders_column = {
                    let volume_slider = create_settings_slider(
                        icons::volume_high(),
                        "Volume",
                        state.volume_enabled,
                        Message::VolumeToggled,
                        0.0..=1.0,
                        state.get_global_volume() as f64,
                        |volume: f64| Message::VolumeChanged(volume as f32),
                    );

                    let speed_slider = create_settings_slider(
                        icons::speed(),
                        "Speed",
                        state.speed_enabled,
                        Message::SpeedToggled,
                        0.0..=2.0,
                        state.get_global_speed(),
                        Message::SpeedChanged,
                    );

                    widget::column!(volume_slider, speed_slider)
                        .spacing(SPACING_SMALL)
                        .width(Length::Fill)
                };

                let stop_button = widget::button(
                    widget::text("Stop All")
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .vertical_alignment(alignment::Vertical::Center)
                        .height(Length::Fill),
                )
                .style(style::button::CustomButton::default())
                .width(Length::Fixed(128.0))
                .height(Length::Fill)
                .on_press(Message::StopAllPlaybacks);

                let row = widget::row!(sliders_column.width(Length::FillPortion(2)), stop_button)
                    .spacing(SPACING_NORMAL)
                    .align_items(Alignment::Center);

                widget::container(row)
                    .width(Length::Fill)
                    .height(Length::Fixed(76.0))
                    .padding(SPACING_NORMAL)
                    .style(style::container::CustomContainer::default())
                    .into()
            }
            Self::Loading => unreachable!(),
        }
    }

    fn view_content(&self) -> Element<Message> {
        match self {
            Self::Loaded(state) => {
                if let Some(tab) = state.get_current_tab() {
                    let clips = tab
                        .clips
                        .iter()
                        .enumerate()
                        .fold(widget::Column::new(), |column, (idx, clip)| {
                            column.push(
                                widget::button(
                                    widget::row!(
                                        widget::text(clip.name.as_str()),
                                        widget::horizontal_space(),
                                        widget::text(format_seconds_to_time(
                                            clip.duration.as_secs_f64()
                                        )),
                                    )
                                    .height(Length::Fill)
                                    .align_items(Alignment::Center),
                                )
                                .width(Length::Fill)
                                .height(Length::Fixed(48.0))
                                .padding([0, SPACING_NORMAL])
                                .on_press(Message::StartPlayback(clip.clone()))
                                .style(style::button::CustomButton::flat()),
                            )
                        })
                        .padding([0, SPACING_NORMAL]);

                    widget::container(
                        widget::scrollable(clips)
                            .height(Length::Fill)
                            .direction(Direction::Vertical(scrollable_properties()))
                            .style(style::scrollable::CustomScrollable::default()),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding([SPACING_NORMAL, 0])
                    .style(
                        style::container::CustomContainer::default().with_border_radius([
                            0.0,
                            BORDER_RADIUS,
                            BORDER_RADIUS,
                            BORDER_RADIUS,
                        ]),
                    )
                    .into()
                } else {
                    widget::container(widget::text("Please create a tab"))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x()
                        .center_y()
                        .style(style::container::CustomContainer::default())
                        .into()
                }
            }
            Self::Loading => unreachable!(),
        }
    }

    fn view_playbacks(&self) -> Element<Message> {
        match self {
            Self::Loaded(state) => {
                let playbacks = state
                    .active_playbacks
                    .iter()
                    .rev()
                    .fold(widget::Column::new(), |column, (id, playback)| {
                        let title =
                            widget::text(truncate_text(playback.clip.name.as_str(), 16).as_str())
                                .width(Length::Fixed(128.0));

                        let play_button = widget::button(icons::play())
                            .width(TOOL_BUTTON_SIZE)
                            .height(TOOL_BUTTON_SIZE)
                            .on_press(Message::AudioEvent(*id, AudioCommand::Play))
                            .style(style::button::CustomButton::active());

                        let pause_button = widget::button(icons::pause())
                            .width(TOOL_BUTTON_SIZE)
                            .height(TOOL_BUTTON_SIZE)
                            .on_press(Message::AudioEvent(*id, AudioCommand::Pause))
                            .style(style::button::CustomButton::flat());

                        let control_button = if playback.handle.state() == PlaybackState::Playing {
                            pause_button
                        } else {
                            play_button
                        };

                        let slider = widget::slider(
                            0.0..=playback.clip.duration.as_secs_f64(),
                            playback.handle.position(),
                            |value| Message::AudioEvent(*id, AudioCommand::Seek(value)),
                        )
                        .step(0.001)
                        .style(style::slider::CustomSlider::default());

                        let playback_position =
                            widget::text(format_seconds_to_time(playback.handle.position()));

                        let playback_duration = widget::text(format_seconds_to_time(
                            playback.clip.duration.as_secs_f64(),
                        ));

                        let stop_button = widget::button(icons::stop())
                            .width(TOOL_BUTTON_SIZE)
                            .height(TOOL_BUTTON_SIZE)
                            .on_press(Message::AudioEvent(*id, AudioCommand::Stop))
                            .style(style::button::CustomButton::flat());

                        column.push(
                            widget::row!(
                                title,
                                control_button,
                                playback_position,
                                slider,
                                playback_duration,
                                stop_button
                            )
                            .spacing(SPACING_NORMAL)
                            .align_items(Alignment::Center),
                        )
                    })
                    .spacing(SPACING_SMALL);

                widget::container(
                    widget::scrollable(playbacks)
                        .direction(Direction::Vertical(scrollable_properties()))
                        .style(style::scrollable::CustomScrollable::default()),
                )
                .width(Length::Fill)
                .max_height(256)
                .padding(SPACING_NORMAL)
                .style(style::container::CustomContainer::default())
                .into()
            }
            Self::Loading => unreachable!(),
        }
    }
}

fn create_settings_slider<'a>(
    icon: Element<'a, Message>,
    label_text: &str,
    enabled: bool,
    on_press_message: Message,
    range: std::ops::RangeInclusive<f64>,
    value: f64,
    on_change_message: impl Fn(f64) -> Message + 'static,
) -> widget::Row<'a, Message> {
    let label_row = {
        let button = widget::button(icon)
            .width(TOOL_BUTTON_SIZE)
            .height(TOOL_BUTTON_SIZE)
            .style(if enabled {
                style::button::CustomButton::flat()
            } else {
                style::button::CustomButton::active().into()
            })
            .on_press(on_press_message);

        let label = widget::text(label_text).width(Length::Fixed(56.0));

        widget::row!(button, label)
            .width(Length::Shrink)
            .spacing(SPACING_SMALL)
            .align_items(Alignment::Center)
    };

    let slider = widget::slider(range, value, on_change_message)
        // .width(Length::Fixed(256.0))
        .step(0.01)
        .on_release(Message::SetDirty)
        .style(style::slider::CustomSlider::default());

    let value_text = widget::text(format!("{:.0}%", value * 100.0))
        .horizontal_alignment(alignment::Horizontal::Left);

    widget::row!(label_row, slider, value_text)
        .width(Length::Fixed(384.0))
        .spacing(SPACING_NORMAL)
        .align_items(Alignment::Center)
}

fn format_seconds_to_time(seconds: f64) -> String {
    let total_seconds = seconds as u64;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}", minutes, seconds)
}

fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() > max_length && max_length > 3 {
        format!("{}...", &text[..max_length - 3])
    } else {
        text.to_string()
    }
}

fn scrollable_properties() -> Properties {
    Properties::default().scroller_width(4.0).width(4)
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

pub fn update(state: &mut AppState, message: &Message) -> Command<Message> {
    let command = match message {
        Message::Saved(_) => {
            println!("Saved!");
            state.save();

            Command::none()
        }
        Message::SelectTab(index) => {
            println!("Tab selected: {}", index);

            state.select_tab(*index);
            state.set_dirty();

            if state.get_current_tab().unwrap().clips.is_empty() {
                println!("Tab is empty, refreshing clips...");
                state.refresh_clips(); // TODO: move to async
            }

            Command::none()
        }
        Message::CloseTab(index) => {
            println!("Tab closed: {}", index);

            state.close_tab(*index);
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
                    directory: path.to_path_buf(),
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
        _ => Command::none(),
    };

    let save = if state.dirty && !state.saving {
        state.saving = true;

        Command::perform(
            SavedState {
                tabs: state.tabs.to_vec(),
                current_tab: state.current_tab,
                global_volume: state.get_global_volume(),
                global_speed: state.get_global_speed(),
            }
            .save(),
            Message::Saved,
        )
    } else {
        Command::none()
    };

    Command::batch(vec![command, save])
}
