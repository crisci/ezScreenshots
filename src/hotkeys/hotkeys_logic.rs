use std::fmt;
use std::path::Display;

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

#[derive(Debug, Clone, PartialEq, Default)]
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

    pub fn get_save_as(&self) -> char {
        self.saveas
    }

    pub fn get_delete(&self) -> char {
        self.delete
    }

    pub fn get_exit(&self) -> char {
        self.exit
    }

    pub fn get_copy(&self) -> char {
        self.copy
    }

    pub fn get_settings(&self) -> char {
        self.settings
    }

    pub fn get_resize(&self) -> char {
        self.resize
    }

    pub fn get_screenshot(&self) -> char {
        self.screenshot
    }

        // Setter methods
    pub fn set_save(&mut self, save: char) {
        self.save = save;
    }

    pub fn set_saveas(&mut self, saveas: char) {
        self.saveas = saveas;
    }

    pub fn set_delete(&mut self, delete: char) {
        self.delete = delete;
    }

    pub fn set_exit(&mut self, exit: char) {
        self.exit = exit;
    }

    pub fn set_copy(&mut self, copy: char) {
        self.copy = copy;
    }

    pub fn set_settings(&mut self, settings: char) {
        self.settings = settings;
    }

    pub fn set_resize(&mut self, resize: char) {
        self.resize = resize;
    }

    pub fn set_screenshot(&mut self, screenshot: char) {
        self.screenshot = screenshot;
    }
    

    pub fn char_already_used(&self, c: char) -> bool {
        if 
        self.save == c || self.saveas == c || self.copy == c || 
        self.delete == c || self.exit == c || self.settings == c || 
        self.resize == c || self.screenshot == c {
            return true;
        }
        return false;
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
            ' ' => Some("Space".to_string()),
            _ => None,
        }
    }
}

impl fmt::Display for HotkeysMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HotkeysMap::Save => write!(f, "Save"),
            HotkeysMap::SaveAs => write!(f, "Save As"),
            HotkeysMap::Delete => write!(f, "Delete"),
            HotkeysMap::Exit => write!(f, "Exit"),
            HotkeysMap::Copy => write!(f, "Copy"),
            HotkeysMap::Settings => write!(f, "Settings"),
            HotkeysMap::Resize => write!(f, "Resize"),
            HotkeysMap::Screenshot => write!(f, "Screenshot"),
            HotkeysMap::None => write!(f, "None"),
        }
    }
}

