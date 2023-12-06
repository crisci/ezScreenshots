use iced::{Alignment, Length};
use iced::Length::Fill;
use iced::mouse::Button;
use iced::widget::{button, Column, container, Row, row, Text, text};
use iced_aw::{Card};
use crate::app::{App, Message};
use crate::custom_widgets::rounded_button;
use crate::hotkeys::hotkeys_logic;

pub fn hotkeys_modal(app: &App) -> Option<Card<Message>> {
    return
        Some(
            Card::new(
                Text::new("Hotkeys"),
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .push(Row::new().push(Text::new("Save")).push(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.hotkeys().get_save()).unwrap())).on_press(Message::CloseModal)).width(Fill).align_items(Alignment::Center)),
            )
                .foot(
                    Row::new()
                        .spacing(10)
                        .padding(5)
                        .width(Length::Fill)
                        .width(Length::Fill)
                        .push(
                            rounded_button("Cancel", Message::CloseModal)
                        )
                        .push(
                            rounded_button("Save", Message::CloseModal)
                        )
                )
                .max_width(300.0)
                //.width(Length::Shrink)
                .on_close(Message::CloseModal),
        );
}
