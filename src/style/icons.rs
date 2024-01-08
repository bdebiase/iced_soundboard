use crate::app::Message;

use iced::Element;

use super::FONT_SIZE_ICON;

fn icon<'a>(codepoint: char) -> iced::widget::Text<'a> {
    iced::widget::text(codepoint)
        .font(iced_aw::graphics::icons::ICON_FONT)
        .size(FONT_SIZE_ICON)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center)
}

pub fn play<'a>() -> Element<'a, Message> {
    icon(iced_aw::graphics::icons::icon_to_char(
        iced_aw::Icon::PlayFill,
    ))
    .into()
}

pub fn pause<'a>() -> Element<'a, Message> {
    icon(iced_aw::graphics::icons::icon_to_char(
        iced_aw::Icon::PauseFill,
    ))
    .into()
}

pub fn stop<'a>() -> Element<'a, Message> {
    icon(iced_aw::graphics::icons::icon_to_char(
        iced_aw::Icon::StopFill,
    ))
    .into()
}

pub fn volume_up<'a>() -> Element<'a, Message> {
    icon(iced_aw::graphics::icons::icon_to_char(
        iced_aw::Icon::VolumeDownFill,
    ))
    .size(FONT_SIZE_ICON + 8)
    .into()
}

pub fn plus<'a>() -> Element<'a, Message> {
    icon(iced_aw::graphics::icons::icon_to_char(iced_aw::Icon::Plus))
        .size(FONT_SIZE_ICON + 5)
        .into()
}

pub fn refresh<'a>() -> Element<'a, Message> {
    icon(iced_aw::graphics::icons::icon_to_char(
        iced_aw::Icon::ArrowClockwise,
    ))
    .into()
}

#[allow(unused)]
pub fn repeat<'a>() -> Element<'a, Message> {
    icon(iced_aw::graphics::icons::icon_to_char(
        iced_aw::Icon::ArrowRepeat,
    ))
    .into()
}

pub fn speed<'a>() -> Element<'a, Message> {
    icon(iced_aw::graphics::icons::icon_to_char(
        iced_aw::Icon::StopwatchFill,
    ))
    .size(FONT_SIZE_ICON - 1)
    .into()
}

pub fn cancel<'a>() -> Element<'a, Message> {
    icon(iced_aw::graphics::icons::icon_to_char(iced_aw::Icon::X))
        .size(FONT_SIZE_ICON + 2)
        .into()
}

#[allow(unused)]
pub fn download<'a>() -> Element<'a, Message> {
    icon(iced_aw::graphics::icons::icon_to_char(
        iced_aw::Icon::Download,
    ))
    .into()
}
