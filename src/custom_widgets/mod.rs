mod custom_button;
mod custom_container;

use iced::{widget::{svg, Container, column, text, container, button}, Renderer, Length, color, Alignment, theme, Color, application};
use iced::alignment::Horizontal;
use iced::application::Appearance;
use iced::widget::container::StyleSheet;
use iced::widget::Text;


use crate::custom_widgets::custom_button::RadiusButton;
use crate::app::Message;
use crate::custom_widgets::custom_container::{CustomContainer};

#[derive(Default)]
pub struct CustomTheme {}

#[derive(Default)]
pub enum CustomThemeStyle {
    #[default]
    Default,
}

impl application::StyleSheet for CustomTheme{
    type Style = CustomThemeStyle;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background_color: Color::BLACK,
            text_color: Color::WHITE,
        }
    }
}

pub fn rounded_button(label: &str, message: Message) -> iced::widget::Button<Message> {
    button(Text::new(label).horizontal_alignment(Horizontal::Center).style(theme::Text::Color(Color::WHITE)))
        .style(iced::theme::Button::Custom(Box::new(RadiusButton::new(label.to_string()))))
        .width(Length::Fill)
        .on_press(message)
}

pub fn rounded_container<'a>(text: String) -> iced::widget::Container<'a, Message, Renderer> {
    container(Text::new(text.clone()).horizontal_alignment(Horizontal::Left))
        .style(iced::theme::Container::Custom(Box::new(CustomContainer::new())))
}

pub fn image_button<'a>(image_name: &'a str, description: &'static str, message: Message) -> Container<'a, Message, Renderer> {
    let handle = svg::Handle::from_path(format!(
        "{}/resources/{}.svg",
        env!("CARGO_MANIFEST_DIR"),
        image_name
    ));

    let svg = svg(handle).width(Length::Fill).height(Length::Fill)
        .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
            color: Some(color!(0xffffff)),
        }));
    let (h, w, p) = match description{
        "Screenshot" => (80, 55, 0),
        "Folder" => (38, 32, 1),
        _ => (55, 55, 5)
    };
    let c = column![
        if description != "Folder" {text(description)} else {text("")},
        container(
            button(container(svg).padding(p))
            .style(iced::theme::Button::Custom(Box::new(RadiusButton::new(description.to_string()))))
            .on_press(message)
            .width(h)
            .height(w)
        ),
    ].align_items(Alignment::Center);

    container(c).center_x()
}

