use iced::{Alignment, Length};
use iced::alignment::Horizontal;
use iced::Length::Fill;
use iced::widget::{button, Column, container, Row, Text};
use iced_aw::Card;
use crate::app::{App, Message};
use crate::custom_widgets::rounded_button;
use crate::hotkeys::hotkeys_logic::{self, HotkeysMap};

pub fn hotkeys_modal(app: &App) -> Option<Card<Message>> {
    return
        Some(
            Card::new(
                Text::new("Hotkeys"),
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .push(Row::new().push(container(Text::new("Save")).align_x(Horizontal::Left).width(Length::Fixed(100.))).push(container(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_save()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Save))).align_x(Horizontal::Left).width(Length::Fixed(80.))))
                    .push(Row::new().push(container(Text::new("Save as")).align_x(Horizontal::Left).width(Length::Fixed(100.))).push(container(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_save_as()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::SaveAs))).align_x(Horizontal::Left).width(Length::Fixed(80.))))
                    .push(Row::new().push(container(Text::new("Delete")).align_x(Horizontal::Left).width(Length::Fixed(100.))).push(container(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_delete()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Delete))).align_x(Horizontal::Left).width(Length::Fixed(80.))))
                    .push(Row::new().push(container(Text::new("Exit")).align_x(Horizontal::Left).width(Length::Fixed(100.))).push(container(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_exit()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Exit))).align_x(Horizontal::Left).width(Length::Fixed(80.))))
                    .push(Row::new().push(container(Text::new("Copy")).align_x(Horizontal::Left).width(Length::Fixed(100.))).push(container(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_copy()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Copy))).align_x(Horizontal::Left).width(Length::Fixed(80.))))
                    .push(Row::new().push(container(Text::new("Delay")).align_x(Horizontal::Left).width(Length::Fixed(100.))).push(container(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_delay()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Delay))).align_x(Horizontal::Left).width(Length::Fixed(80.))))
                    .push(Row::new().push(container(Text::new("Resize")).align_x(Horizontal::Left).width(Length::Fixed(100.))).push(container(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_resize()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Resize))).align_x(Horizontal::Left).width(Length::Fixed(80.))))
                    .push(Row::new().push(container(Text::new("Screenshot")).align_x(Horizontal::Left).width(Length::Fixed(100.))).push(container(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_screenshot()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Screenshot))).align_x(Horizontal::Left).width(Length::Fixed(80.))))
                    .push( if app.get_hotkey_modification() != HotkeysMap::None {
                        Row::new().push(Text::new(format!("Insert the new combination for {}", app.get_hotkey_modification())))
                    } else {Row::new()})   
                    .push(if app.get_hotkeys_error().is_some() {Row::new().push(Text::new(app.get_hotkeys_error().unwrap()))} else {Row::new()}) 
                    .width(Fill)           
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
                            rounded_button("Save", Message::HotkeysSave)
                        )
                )
                .max_width(300.0)
                //.width(Length::Shrink)
                .on_close(Message::CloseModal),
        );
}
