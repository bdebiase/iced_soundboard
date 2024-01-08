#[derive(Default)]
enum CustomRuleStyle {
    #[default]
    Default,
    Dark,
}

pub struct CustomRule(CustomRuleStyle);

impl Default for CustomRule {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl CustomRule {
    pub fn dark() -> Self {
        Self(CustomRuleStyle::Dark)
    }
}

impl iced::widget::rule::StyleSheet for CustomRule {
    type Style = iced::theme::Theme;

    fn appearance(&self, style: &Self::Style) -> iced::widget::rule::Appearance {
        iced::widget::rule::Appearance {
            color: match self.0 {
                CustomRuleStyle::Default => iced::Color {
                    a: 0.1,
                    ..style.palette().text
                },
                CustomRuleStyle::Dark => iced::Color {
                    a: 1.0,
                    ..iced::Color::BLACK
                },
            },
            fill_mode: iced::widget::rule::FillMode::Full,
            radius: 0.0.into(),
            width: 1,
        }
    }
}

impl std::convert::From<CustomRule> for iced::theme::Rule {
    fn from(value: CustomRule) -> Self {
        iced::theme::Rule::Custom(Box::new(value))
    }
}
