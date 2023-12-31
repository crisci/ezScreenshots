use std::fmt::{Display, Formatter};
use iced::alignment::Horizontal::Center;
use iced::{Alignment, Font, Length};
use iced::widget::{Column, container, Row, Text, text_input};
use iced_aw::{Card, SelectionList, SelectionListStyles};
use iced_aw::native::Spinner;
use crate::app::{App, Message, SaveState};
use crate::custom_widgets::{image_button, rounded_button, rounded_container};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Formats {
    #[default]
    Png,
    Gif,
    Jpeg
}


impl Formats {
    pub const ALL: [Formats; 3] = [
        Formats::Png,
        Formats::Jpeg,
        Formats::Gif
    ];
}

impl From<String> for Formats {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Jpeg" => Formats::Jpeg,
            "Png" => Formats::Png,
            "Gif" => Formats::Gif,
            _ => panic!("Format not valid")
        }
    }
}

impl Display for Formats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Formats::Gif => "Gif",
                Formats::Jpeg => "Jpeg",
                Formats::Png => "Png"
            }
        )
    }
}


pub fn save_as_modal<'a>(app: &'a App) -> Option<Card<'a, Message>> {
    let selection_list: SelectionList<_, Message> = SelectionList::new_with(
        &app.formats()[..],
        Message::FormatSelected,
        16.0,
        5.0,
        SelectionListStyles::Default,
        app.manual_select(),
        Font::default(),
    )
        .width(Length::Shrink)
        .height(Length::Shrink);
    let foot_row = Row::new()
        .spacing(10)
        .padding(5)
        .width(Length::Fill);

    let choose_path = container(Row::new()
        .spacing(10)
        .push(rounded_container(app.save_path()).width(Length::FillPortion(6)))
        .push(image_button("folder","Folder", Message::PathSelected).width(Length::FillPortion(1)))
        .spacing(10)
        .align_items(Alignment::End))
        .center_x()
        .center_y();

    let choose_name = container(Row::new()
        .spacing(10)
        .push(text_input("", &app.save_name())
            .on_input(Message::NameChanges)
            .on_submit(Message::SaveAsButtonPressed)
            .padding(15)))
        .center_x()
        .center_y();


    return
        Some(
            Card::new(
                Text::new("Save as..."),
                Column::new()
                    .spacing(10)
                    .push(Text::new("Select the output format").width(Length::Fill).horizontal_alignment(Center))
                    .push(selection_list)
                    .push(Text::new("Select the output folder"))
                    .push(choose_path)
                    .push(Text::new("Select the name"))
                    .push(choose_name)
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
            )
                .foot(
                    match app.save_state() {
                        SaveState::Nothing => foot_row
                            .push(
                                rounded_button("Cancel", Message::CloseModal)
                            )
                            .push(
                                rounded_button("Save", Message::SaveAsButtonPressed)
                            ),
                        SaveState::OnGoing => foot_row.push(container(Spinner::new())
                            .width(Length::Fill)
                            .center_x()
                            .center_y()),
                    },
                )
                .max_width(300.0)
                //.width(Length::Shrink)
                .on_close(Message::CloseModal),
        )
}
