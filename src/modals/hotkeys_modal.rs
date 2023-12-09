use iced::{Alignment, Length};
use iced::Length::Fill;
use iced::mouse::Button;
use iced::widget::{button, Column, container, Row, row, Text, text};
use iced_aw::{Card};
use tracing_subscriber::fmt::format;
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
                    .push(Row::new().push(Text::new("Save")).push(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_save()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Save))))
                    .push(Row::new().push(Text::new("Save as")).push(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_save_as()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::SaveAs))))
                    .push(Row::new().push(Text::new("Delete")).push(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_delete()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Delete))))
                    .push(Row::new().push(Text::new("Exit")).push(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_exit()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Exit))))
                    .push(Row::new().push(Text::new("Copy")).push(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_copy()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Copy))))
                    .push(Row::new().push(Text::new("Settings")).push(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_settings()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Settings))))
                    .push(Row::new().push(Text::new("Resize")).push(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_resize()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Resize))))
                    .push(Row::new().push(Text::new("Screenshot")).push(button(Text::new(hotkeys_logic::Hotkeys::unicode_to_str(app.temp_hotkeys().get_screenshot()).unwrap())).on_press(Message::ChangeHotkey(HotkeysMap::Screenshot)))) 
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
