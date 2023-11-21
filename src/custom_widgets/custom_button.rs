use iced::{color, Color, Theme};
use std::default::Default;
use iced::widget::{button, button::Appearance};

#[derive(Default)]
pub struct RadiusButton {
    label: String
}

impl RadiusButton {
    pub fn new(label: String) -> Self {
        Self {label}
    }
}
impl button::StyleSheet for RadiusButton {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
            return match self.label.as_str() {
                "Resize" => resize_theme(),
                "Delete" => delete_theme(),
                "Save" => save_theme(),
                "Cancel" => cancel_theme(),
                _ => screenshot_theme()
            }
    }
}

fn screenshot_theme() -> Appearance {
    Appearance {
        border_radius: 18.0.into(),
        background: Option::from(iced::Background::Color(Color::from(color!(0x364F6B)))),
        ..Appearance::default()
    }
}

fn delete_theme() -> Appearance {
    Appearance {
        border_radius: 100.0.into(),
        background: Option::from(iced::Background::Color(Color::from(color!(0xF90851)))),
        ..Appearance::default()
    }
}

fn resize_theme() -> Appearance {
    Appearance {
        border_radius: 100.0.into(),
        background: Option::from(iced::Background::Color(Color::from(color!(0xFF8328)))),
        ..Appearance::default()
    }
}

fn save_theme() -> Appearance {
    Appearance {
        border_radius:15.0.into(),
        background: Option::from(iced::Background::Color(Color::from(color!(0x64BF45)))),
        ..Appearance::default()
    }
}

fn cancel_theme() -> Appearance {
    Appearance {
        border_radius:15.0.into(),
        background: Option::from(iced::Background::Color(Color::from(color!(0xF90851)))),
        ..Appearance::default()
    }
}