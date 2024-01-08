use super::BORDER_RADIUS;

#[derive(Default)]
enum CustomSliderStyle {
    #[default]
    Default,
}

#[derive(Default)]
pub struct CustomSlider(CustomSliderStyle);

impl std::convert::From<CustomSlider> for iced::theme::Slider {
    fn from(value: CustomSlider) -> Self {
        iced::theme::Slider::Custom(Box::new(value))
    }
}

impl iced::widget::slider::StyleSheet for CustomSlider {
    type Style = iced::theme::Theme;

    fn active(&self, style: &Self::Style) -> iced::widget::slider::Appearance {
        let mut rail_color = style.palette().primary;
        rail_color.a = 0.5;

        iced::widget::slider::Appearance {
            handle: iced::widget::slider::Handle {
                shape: iced::widget::slider::HandleShape::Circle { radius: 6.0 },
                color: style.palette().primary,
                border_width: 0.0,
                border_color: iced::Color::TRANSPARENT,
            },
            rail: iced::widget::slider::Rail {
                width: 4.0,
                colors: (rail_color, iced::Color::from_rgba(1.0, 1.0, 1.0, 0.25)),
                border_radius: BORDER_RADIUS.into(),
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::slider::Appearance {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> iced::widget::slider::Appearance {
        self.active(style)
    }
}
