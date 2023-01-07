use std::default;

use iced::{widget::button, Color};

#[derive(Default, Clone, Debug)]
pub enum ButtonStyle {
    #[default]
    Red,
}

impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(255, 228, 196))),
            border_radius: 5.0,
            ..button::Appearance::default()
        }
    }
}
