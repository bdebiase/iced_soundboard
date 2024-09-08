use std::{collections::HashMap, sync::Mutex};

use crate::app::Message;
use crate::style;

use iced::{widget::svg::Handle, Element};
use lazy_static::lazy_static;

use super::{ICON_DIR, ICON_SIZE};

const DEFAULT_ICON_BYTES: &[u8] =
    include_bytes!("../../assets/adwaita-icons/emblem-important-symbolic.svg");

lazy_static! {
    static ref ICON_CACHE: Mutex<HashMap<String, Handle>> = Mutex::new(HashMap::new());
}

fn icon<'a>(name: &str) -> iced::widget::svg::Svg {
    let handle = load_icon(name).unwrap_or_else(|| Handle::from_memory(DEFAULT_ICON_BYTES));

    iced::widget::svg(handle)
        .width(iced::Length::Fixed(ICON_SIZE))
        .height(iced::Length::Fixed(ICON_SIZE))
        .style(style::svg::CustomSvg::default())
}

fn load_icon(name: &str) -> Option<Handle> {
    let mut cache = ICON_CACHE.lock().unwrap();
    if let Some(handle) = cache.get(name) {
        return Some(handle.clone());
    }

    let file_name = name.to_string() + ".svg";
    let contents = ICON_DIR
        .get_file(&file_name)
        .and_then(|file| Some(file.contents()))?;

    let handle = Handle::from_memory(contents);
    cache.insert(name.to_string(), handle.clone());
    Some(handle)
}

pub fn play<'a>() -> Element<'a, Message> {
    icon("media-playback-start-symbolic").into()
}

pub fn pause<'a>() -> Element<'a, Message> {
    icon("media-playback-pause-symbolic").into()
}

pub fn stop<'a>() -> Element<'a, Message> {
    icon("media-playback-stop-symbolic").into()
}

pub fn volume_high<'a>() -> Element<'a, Message> {
    icon("audio-volume-high-symbolic").into()
}

#[allow(unused)]
pub fn volume_medium<'a>() -> Element<'a, Message> {
    icon("audio-volume-medium-symbolic").into()
}

#[allow(unused)]
pub fn volume_low<'a>() -> Element<'a, Message> {
    icon("audio-volume-low-symbolic").into()
}

#[allow(unused)]
pub fn volume_muted<'a>() -> Element<'a, Message> {
    icon("audio-volume-muted-symbolic").into()
}

pub fn plus<'a>() -> Element<'a, Message> {
    icon("list-add-symbolic").into()
}

pub fn refresh<'a>() -> Element<'a, Message> {
    icon("view-refresh-symbolic").into()
}

#[allow(unused)]
pub fn repeat<'a>() -> Element<'a, Message> {
    icon("media-playlist-repeat-symbolic").into()
}

pub fn speed<'a>() -> Element<'a, Message> {
    icon("power-profile-performance-symbolic").into()
}

pub fn cancel<'a>() -> Element<'a, Message> {
    icon("window-close-symbolic").into()
}

#[allow(unused)]
pub fn download<'a>() -> Element<'a, Message> {
    icon("folder-download-symbolic").into()
}

#[allow(unused)]
pub fn settings<'a>() -> Element<'a, Message> {
    icon("emblem-system-symbolic").into()
}
