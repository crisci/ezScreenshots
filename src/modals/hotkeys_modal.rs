use std::fmt::{Display};
use iced::{Alignment, Length};
use iced::widget::{Column, Row, Text};
use iced_aw::{Card};
use crate::app::{App, Message};
use crate::custom_widgets::rounded_button;

pub fn hotkeys_modal(app: &App) -> Option<Card<Message>> {
    let foot_row = Row::new()
        .spacing(10)
        .padding(5)
        .width(Length::Fill);
    return
        Some(
            Card::new(
                Text::new("Hotkeys"),
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
            )
                .foot(
                   foot_row
                            .push(
                                rounded_button("Cancel", Message::CancelButtonPressed)
                            )
                            .push(
                                rounded_button("Save", Message::CloseModal)
                            )
                )
                .max_width(300.0)
                //.width(Length::Shrink)
                .on_close(Message::CloseModal),
        )
}