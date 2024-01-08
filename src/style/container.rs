use super::BORDER_RADIUS;

#[allow(unused)]
struct CustomContainerState {
    custom_style: CustomContainerStyle,
    border_radius: iced::BorderRadius,
}

impl Default for CustomContainerState {
    fn default() -> Self {
        Self {
            custom_style: CustomContainerStyle::Default,
            border_radius: [BORDER_RADIUS; 4].into(),
        }
    }
}

#[derive(Default)]
enum CustomContainerStyle {
    #[default]
    Default,
}

#[derive(Default)]
pub struct CustomContainer(CustomContainerState);

impl CustomContainer {
    pub fn with_border_radius(mut self, radius: [f32; 4]) -> Self {
        self.0.border_radius = radius.into();
        self
    }
}

impl std::convert::From<CustomContainer> for iced::theme::Container {
    fn from(value: CustomContainer) -> Self {
        iced::theme::Container::Custom(Box::new(value))
    }
}

impl iced::widget::container::StyleSheet for CustomContainer {
    type Style = iced::theme::Theme;

    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        match &self.0 {
            CustomContainerState {
                custom_style: _,
                border_radius,
            } => {
                let mut background = style.palette().primary;
                background.a = 0.05;

                iced::widget::container::Appearance {
                    background: Some(background.into()),
                    border_radius: *border_radius,
                    border_color: iced::Color::TRANSPARENT,
                    border_width: 0.0,
                    ..Default::default()
                }
            }
        }
    }
}
