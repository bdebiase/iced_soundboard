use iced::Border;

use super::BORDER_RADIUS;

#[derive(Default)]
enum CustomScrollableStyle {
    #[default]
    Default,
}

#[derive(Default)]
pub struct CustomScrollable(CustomScrollableStyle);

impl std::convert::From<CustomScrollable> for iced::theme::Scrollable {
    fn from(value: CustomScrollable) -> Self {
        iced::theme::Scrollable::custom(value)
    }
}

impl iced::widget::scrollable::StyleSheet for CustomScrollable {
    type Style = iced::theme::Theme;

    fn active(&self, style: &Self::Style) -> iced::widget::scrollable::Appearance {
        iced::widget::scrollable::Appearance {
            scrollbar: iced::widget::scrollable::Scrollbar {
                background: None,
                border: iced::Border::default(),
                scroller: iced::widget::scrollable::Scroller {
                    color: {
                        let mut color = style.palette().primary;
                        color.a = 0.05;
                        color
                    },
                    border: Border {
                        radius: BORDER_RADIUS.into(),
                        ..Default::default()
                    },
                },
            },
            container: iced::widget::container::Appearance::default(),
            gap: None,
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> iced::widget::scrollable::Appearance {
        iced::widget::scrollable::Appearance {
            scrollbar: iced::widget::scrollable::Scrollbar {
                background: None,
                border: iced::Border::default(),
                scroller: iced::widget::scrollable::Scroller {
                    color: {
                        let mut color = style.palette().primary;
                        color.a = if is_mouse_over_scrollbar { 0.125 } else { 0.05 };
                        color
                    },
                    border: Border {
                        radius: BORDER_RADIUS.into(),
                        ..Default::default()
                    },
                },
            },
            container: iced::widget::container::Appearance::default(),
            gap: None,
        }
    }
}
