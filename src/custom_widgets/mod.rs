mod custom_button;

use iced::{widget::{svg, Container, column, text, container, button}, Renderer, Length, color, Alignment, mouse::Button, theme::Svg, theme, Color, application};
use iced::application::Appearance;


use crate::custom_widgets::custom_button::RadiusButton;
use crate::capture::Message;

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
    let (h, w) = if description == "Screenshot" { (70, 55) } else { (55, 55) };
    let c = column![

        text(description),
        container(
            button(svg)
            .style(iced::theme::Button::Custom(Box::new(RadiusButton::new(description.to_string()))))
            .on_press(message)
            .width(h)
            .height(w)
        ),
    ].align_items(Alignment::Center);

    container(c).center_x()
}

