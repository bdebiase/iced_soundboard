#[derive(Default)]
enum CustomSvgStyle {
    #[default]
    Default,
}

#[derive(Default)]
pub struct CustomSvg(CustomSvgStyle);

impl std::convert::From<CustomSvg> for iced::theme::Svg {
    fn from(value: CustomSvg) -> Self {
        iced::theme::Svg::Custom(Box::new(value))
    }
}

impl iced::widget::svg::StyleSheet for CustomSvg {
    type Style = iced::theme::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::svg::Appearance {
        iced::widget::svg::Appearance {
            color: Some(iced::Color::from_linear_rgba(1.0, 1.0, 1.0, 1.0)),
        }
    }
}
