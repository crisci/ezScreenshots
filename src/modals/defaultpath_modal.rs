use iced::{Alignment, Length};
use iced::widget::{Column, Row, Text,container};
use iced_aw::Card;
use crate::app::{App, Message};
use crate::custom_widgets::{image_button, rounded_button, rounded_container};
use crate::utils::utils::format_path;

pub fn setpath_modal(app: &App) -> Option<Card<Message>> {
    let choose_path = container(Row::new()
        .spacing(10)
        .push(rounded_container(format_path(app.save_path(),27)).width(Length::FillPortion(6)))
        .push(image_button("folder","Folder", Message::PathSelected).width(Length::FillPortion(1)))
        .spacing(10)
        .align_items(Alignment::End))
        .center_x()
        .center_y();
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
                            rounded_button("Save", Message::SetDefaultPath)
                        )
                )
                .max_width(300.0)
                //.width(Length::Shrink)
                .on_close(Message::CloseModal),
        )
}