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
            settings: '\u{10}', //CTRL+P
            resize: '\u{12}', //CTRL+R
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

    pub fn get_save(&self) -> char {
        self.save
    }

    pub fn unicode_to_str(c: char) -> Option<String> {
        match c {
            '\u{1}' => Some("CTRL + A".to_string()),
            '\u{2}' => Some("CTRL + B".to_string()),
            '\u{3}' => Some("CTRL + C".to_string()),
            '\u{4}' => Some("CTRL + D".to_string()),
            '\u{5}' => Some("CTRL + E".to_string()),
            '\u{6}' => Some("CTRL + F".to_string()),
            '\u{7}' => Some("CTRL + G".to_string()),
            '\u{8}' => Some("CTRL + H".to_string()),
            '\u{9}' => Some("CTRL + I".to_string()),
            '\u{A}' => Some("CTRL + J".to_string()),
            '\u{B}' => Some("CTRL + K".to_string()),
            '\u{C}' => Some("CTRL + L".to_string()),
            '\u{D}' => Some("CTRL + M".to_string()),
            '\u{E}' => Some("CTRL + N".to_string()),
            '\u{F}' => Some("CTRL + O".to_string()),
            '\u{10}' => Some("CTRL + P".to_string()),
            '\u{11}' => Some("CTRL + Q".to_string()),
            '\u{12}' => Some("CTRL + R".to_string()),
            '\u{13}' => Some("CTRL + S".to_string()),
            '\u{14}' => Some("CTRL + T".to_string()),
            '\u{15}' => Some("CTRL + U".to_string()),
            '\u{16}' => Some("CTRL + V".to_string()),
            '\u{17}' => Some("CTRL + W".to_string()),
            '\u{18}' => Some("CTRL + X".to_string()),
            '\u{19}' => Some("CTRL + Y".to_string()),
            '\u{1A}' => Some("CTRL + Z".to_string()),
            _ => None,
        }
    }
}


