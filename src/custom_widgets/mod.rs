use iced::{widget::{svg, Container, column, text, container, button}, Renderer, Length, color, theme, Alignment};

use crate::capture::Message;

pub fn image_button<'a>(image_name: &'a str, description: &'a str, message: Message) -> Container<'a, Message, Renderer> {
    let handle = svg::Handle::from_path(format!(
        "{}/resources/{}.svg",
        env!("CARGO_MANIFEST_DIR"),
        image_name
    ));

    let svg = svg(handle).width(Length::Fill).height(Length::Fill)
        .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
            color: Some(color!(0xffffff)),
        }));
    let c = column![
        text(description),
        container(
            button(svg)
            .on_press(message)
            .style(theme::Button::Primary)
            .width(65)
            .height(50)
        ),
    ].align_items(Alignment::Center);

    container(c).center_x()
}