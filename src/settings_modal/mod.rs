use std::fmt::{Display, Formatter};
use iced::alignment::Horizontal::Center;
use iced::{Alignment, Font, Length};
use iced::widget::{Column, container, Row, Text};
use iced_aw::{Card, number_input, NumberInputStyles, SelectionList, SelectionListStyles};
use iced_aw::native::Spinner;
use crate::app::{App, Message, SaveState};
use crate::custom_widgets::rounded_button;

pub fn settings_modal<'a>(app: &'a App) -> Option<Card<'a, Message>> {
    let txt_seconds = number_input(app.temp(), 100., Message::DelayChanged)
        .style(NumberInputStyles::Default)
        .step(1.);
    return
        Some(
            Card::new(
                Text::new("Settings"),
                Row::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Text::new("Delay:"))
                    .push(txt_seconds)
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
                            rounded_button("Save", Message::SettingSave)
                        ),

                )
                .max_width(300.0)
                //.width(Length::Shrink)
                .on_close(Message::CloseModal),
        )
}