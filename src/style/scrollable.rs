use iced::{widget::scrollable::Scroller, Color};

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

    fn active(&self, style: &Self::Style) -> iced::widget::scrollable::Scrollbar {
        iced::widget::scrollable::Scrollbar {
            background: None,
            border_color: Color::TRANSPARENT,
            border_radius: 0.0.into(),
            border_width: 0.0,
            scroller: Scroller {
                color: {
                    let mut color = style.palette().primary;
                    color.a = 0.05;
                    color
                },
                border_radius: BORDER_RADIUS.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> iced::widget::scrollable::Scrollbar {
        iced::widget::scrollable::Scrollbar {
            background: None,
            border_color: Color::TRANSPARENT,
            border_radius: 0.0.into(),
            border_width: 0.0,
            scroller: Scroller {
                color: {
                    let mut color = style.palette().primary;
                    color.a = if is_mouse_over_scrollbar { 0.125 } else { 0.05 };
                    color
                },
                border_radius: BORDER_RADIUS.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}
