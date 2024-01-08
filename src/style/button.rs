use super::BORDER_RADIUS;

struct CustomButtonState {
    custom_style: CustomButtonStyle,
    color: Option<iced::Color>,
    border_radius: iced::BorderRadius,
}

impl Default for CustomButtonState {
    fn default() -> Self {
        Self {
            custom_style: CustomButtonStyle::Default,
            color: None,
            border_radius: [BORDER_RADIUS; 4].into(),
        }
    }
}

#[derive(Default)]
enum CustomButtonStyle {
    #[default]
    Default,
    Flat,
    Active,
    Toolbar,
    Tab(bool),
}

#[derive(Default)]
pub struct CustomButton(CustomButtonState);

impl CustomButton {
    pub fn flat() -> Self {
        Self(CustomButtonState {
            custom_style: CustomButtonStyle::Flat,
            ..Default::default()
        })
    }

    pub fn active() -> Self {
        Self(CustomButtonState {
            custom_style: CustomButtonStyle::Active,
            ..Default::default()
        })
    }

    pub fn toolbar() -> Self {
        Self(CustomButtonState {
            custom_style: CustomButtonStyle::Toolbar,
            ..Default::default()
        })
    }

    pub fn tab(active: bool) -> Self {
        Self(CustomButtonState {
            custom_style: CustomButtonStyle::Tab(active),
            ..Default::default()
        })
    }

    pub fn with_color(mut self, color: iced::Color) -> Self {
        self.0.color = Some(color);
        self
    }

    pub fn with_border_radius(mut self, radius: [f32; 4]) -> Self {
        self.0.border_radius = radius.into();
        self
    }
}

impl std::convert::From<CustomButton> for iced::theme::Button {
    fn from(value: CustomButton) -> Self {
        iced::theme::Button::custom(value)
    }
}

impl iced::widget::button::StyleSheet for CustomButton {
    type Style = iced::theme::Theme;

    fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        match &self.0 {
            CustomButtonState {
                custom_style,
                color,
                border_radius,
            } => match custom_style {
                CustomButtonStyle::Default => {
                    let mut background = if let Some(color) = color {
                        *color
                    } else {
                        style.palette().primary
                    };
                    background.a = 0.05;

                    iced::widget::button::Appearance {
                        background: Some(background.into()),
                        text_color: style.palette().text,
                        border_radius: *border_radius,
                        ..Default::default()
                    }
                }
                CustomButtonStyle::Flat => iced::widget::button::Appearance {
                    text_color: style.palette().text,
                    border_radius: *border_radius,
                    ..Default::default()
                },
                CustomButtonStyle::Active => self.pressed(style),
                CustomButtonStyle::Toolbar => iced::widget::button::Appearance {
                    text_color: style.extended_palette().primary.strong.color,
                    ..Default::default()
                },
                CustomButtonStyle::Tab(active) => {
                    let mut background = style.palette().primary;
                    background.a = if *active { 0.05 } else { 0.025 };

                    let mut text_color = style.palette().text;
                    if !*active {
                        text_color.r -= 0.25;
                        text_color.g -= 0.25;
                        text_color.b -= 0.25;
                    }

                    iced::widget::button::Appearance {
                        background: Some(background.into()),
                        text_color,
                        border_radius: *border_radius,
                        ..Default::default()
                    }
                }
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        match &self.0 {
            CustomButtonState {
                custom_style,
                color,
                border_radius,
            } => match custom_style {
                CustomButtonStyle::Default
                | CustomButtonStyle::Flat
                | CustomButtonStyle::Active => {
                    let mut background = if let Some(color) = color {
                        *color
                    } else {
                        style.palette().primary
                    };
                    background.a = 0.125;

                    iced::widget::button::Appearance {
                        // background: Some(iced::Background::Color(iced::Color::from_rgba(
                        //     1.0, 1.0, 1.0, 0.125,
                        // ))),
                        background: Some(background.into()),
                        text_color: style.palette().text,
                        border_radius: *border_radius,
                        ..Default::default()
                    }
                }
                CustomButtonStyle::Toolbar => iced::widget::button::Appearance {
                    text_color: style.extended_palette().primary.base.color,
                    ..Default::default()
                },
                CustomButtonStyle::Tab(_) => {
                    let mut background = style.palette().primary;
                    background.a = 0.125;

                    iced::widget::button::Appearance {
                        background: Some(background.into()),
                        text_color: style.palette().text,
                        border_radius: *border_radius,
                        ..Default::default()
                    }
                }
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        match self.0 {
            _ => self.hovered(style),
        }
    }
}
