use crate::app::{Message};
use crate::modals::Modals;

#[derive(Debug, Clone, Default)]
pub struct Hotkeys {
    save: char,
    saveas: char,
    delete: char,
    exit: char,
    copy: char,
    settings: char,
    resize: char,
    screenshot: char,
    // TODO: delay screenshot ??
}

#[derive(Debug, Clone, Default)]
pub enum HotkeysMap {
    #[default]
    Save = 0,
    SaveAs = 1,
    Delete = 2,
    Exit = 3,
    Copy = 4,
    Settings = 5,
    Resize = 6,
    Screenshot = 7,
    None = 255
}

impl Hotkeys {
    pub fn new() -> Self {
        Self {
            save: '\u{13}', //CTRL+S
            saveas: '\u{1}', //CTRL+A
            delete: '\u{4}', //CTRL+D
            exit: '\u{5}', //CTRL+E
            copy: '\u{3}', //CTRL+C
            settings: '\u{16}', //CTRL+P
            resize: '\u{18}', //CTRL+R
            screenshot: ' ' //SPACE
        }
    }

}

impl Hotkeys {

    pub fn to_message(&self, c: char) -> Option<Message> {
        return match c {
            _ if self.save == c => Some(Message::MenuAction(Modals::Save)),
            _ if self.saveas == c => Some(Message::MenuAction(Modals::SaveAs)),
            _ if self.delete == c => Some(Message::Drop),
            _ if self.exit == c => None, //TODO: implement close application
            _ if self.copy == c => None, //TODO: copy to clipboard
            _ if self.settings == c => Some(Message::MenuAction(Modals::Settings)),
            _ if self.resize == c => Some(Message::Resize),
            _ if self.screenshot == c => Some(Message::Screenshot),
            _ => None,
        };
    }
}


