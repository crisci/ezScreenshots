use std::fmt::{Display, Formatter};
use iced::alignment::Horizontal::Center;
use iced::{Alignment, Font, Length};
use iced::widget::{Column, container, Row, Text};
use iced_aw::{Card, SelectionList, SelectionListStyles};
use iced_aw::native::Spinner;
use crate::app::{App, Message, SaveState};
use crate::custom_widgets::rounded_button;

pub fn settings_modal<'a>(app: &'a App) -> Option<Card<'a, Message>> {
    return
        Some(
            Card::new(
                Text::new("Settings"),
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Text::new("Delay:").width(Length::Fill).horizontal_alignment(Center))
            )
                .foot(
                    Row::new()
                        .spacing(10)
                        .padding(5)
                        .width(Length::Fill)
                        .push(
                            rounded_button("Cancel", Message::CancelButtonPressed)
                        )
                        .push(
                            rounded_button("Save", Message::SaveAsButtonPressed)
                        ),

                )
                .max_width(300.0)
                //.width(Length::Shrink)
                .on_close(Message::CloseModal),
        )
}