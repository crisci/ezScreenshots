use iced::{Alignment, Length};
use iced::widget::{Column, Row, Text};
use iced_aw::{Card};
use crate::app::{App, Message};
use crate::custom_widgets::{image_button, rounded_button};

pub fn setpath_modal(app: &App) -> Option<Card<Message>> {
    let set_default= true;
    let choose_path = Row::new()
        .spacing(10)
        .push(Text::new(app.save_path()))
        .push(image_button("folder","Folder", Message::PathSelected(set_default)))
        .spacing(10)
        .align_items(Alignment::End);
    let foot_row = Row::new()
        .spacing(10)
        .padding(5)
        .width(Length::Fill);
    return
        Some(
            Card::new(
                Text::new("Default Path"),
                Column::new()
                    .push(choose_path)
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
            )
                .foot(
                    foot_row
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
        )
}