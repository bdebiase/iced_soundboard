#[derive(Clone, Copy, Debug)]
pub struct CustomTheme {
    background: iced::Color,
    text: iced::Color,
    primary: iced::Color,
    success: iced::Color,
    danger: iced::Color,
}

impl Default for CustomTheme {
    fn default() -> Self {
        Self {
            background: iced::Color::from_rgb(0.125, 0.125, 0.125),
            text: iced::Color::from_rgb(0.75, 0.75, 0.75),
            primary: iced::Color::from_rgb(1.0, 1.0, 1.0),
            success: iced::Color::from_rgb(0.25, 0.75, 0.25),
            danger: iced::Color::from_rgb(1.0, 0.0, 0.0),
        }
    }
}

impl std::convert::From<CustomTheme> for iced::theme::Theme {
    fn from(value: CustomTheme) -> Self {
        iced::theme::Theme::custom(
            "".to_string(),
            iced::theme::Palette {
                background: value.background,
                text: value.text,
                primary: value.primary,
                success: value.success,
                danger: value.danger,
            },
        )
    }
}
