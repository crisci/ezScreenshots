use iced::alignment::Horizontal;
use iced::alignment::Horizontal::Center;
use iced::Length;
use iced::widget::{Button, Row, Text};
use iced_aw::Card;
use crate::app::Message;
use crate::custom_widgets::rounded_button;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Formats {
    #[default]
    Png,
    Gif,
    Jpeg
}

pub fn save_as_modal<'a>() -> Option<Card<'a, Message>> {
    return
        Some(
            Card::new(
                Text::new("Save as..."),
                Text::new("Select the output format").width(Length::Fill).horizontal_alignment(Center), //Text::new("Zombie ipsum reversus ab viral inferno, nam rick grimes malum cerebro. De carne lumbering animata corpora quaeritis. Summus brains sit​​, morbo vel maleficia? De apocalypsi gorger omero undead survivor dictum mauris. Hi mindless mortuis soulless creaturas, imo evil stalking monstra adventus resi dentevil vultus comedat cerebella viventium. Qui animated corpse, cricket bat max brucks terribilem incessu zomby. The voodoo sacerdos flesh eater, suscitat mortuos comedere carnem virus. Zonbi tattered for solum oculi eorum defunctis go lum cerebro. Nescio brains an Undead zombies. Sicut malus putrid voodoo horror. Nigh tofth eliv ingdead.")
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
                            rounded_button("Save", Message::OkButtonPressed)
                        ),
                )
                .max_width(300.0)
                //.width(Length::Shrink)
                .on_close(Message::CloseSaveAsModal),
        )
}