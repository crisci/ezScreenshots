use iced::{Alignment, Length};
use iced::widget::{Row, Text};
use iced_aw::{Card, number_input, NumberInputStyles};
use crate::app::{App, Message};
use crate::custom_widgets::rounded_button;

pub fn delay_modal(app: &App) -> Option<Card<Message>> {
    let txt_seconds = number_input(app.temp(), 100., Message::DelayChanged)
        .style(NumberInputStyles::Default)
        .step(1.);
    return
        Some(
            Card::new(
                Text::new("Delay"),
                Row::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Text::new("Set Delay Time(s):"))
                    .push(txt_seconds)
            )
                .foot(
                    Row::new()
                        .spacing(10)
                        .padding(5)
                        .width(Length::Fill)
                        .push(
                            rounded_button("Cancel", Message::CloseModal)
                        )
                        .push(
                            rounded_button("Save", Message::DelaySave)
                        ),

                )
                .max_width(300.0)
                //.width(Length::Shrink)
                .on_close(Message::CloseModal),
        )
}