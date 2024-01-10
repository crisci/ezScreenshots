use std::fs::File;
use std::io::Write;
use std::{fmt, fs};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::Message;
use crate::modals::Modals;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Hotkeys {
    save: char,
    saveas: char,
    delete: char,
    exit: char,
    copy: char,
    delay: char,
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
    Delay = 5,
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
            delay: '\u{10}', //CTRL+P
            resize: '\u{12}', //CTRL+R
            screenshot: ' ' //SPACE
        }
    }

}

impl Hotkeys {

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

    pub fn get_delay(&self) -> char {
        self.delay
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

    pub fn set_delay(&mut self, delay: char) {
        self.delay = delay;
    }

    pub fn set_resize(&mut self, resize: char) {
        self.resize = resize;
    }

    pub fn set_screenshot(&mut self, screenshot: char) {
        self.screenshot = screenshot;
    }

    pub fn to_message(&self, c: char) -> Option<Message> {
        return match c {
            _ if self.save == c => Some(Message::MenuAction(Modals::Save)),
            _ if self.saveas == c => Some(Message::MenuAction(Modals::SaveAs)),
            _ if self.delete == c => Some(Message::Drop),
            _ if self.exit == c => Some(Message::Quit),
            _ if self.copy == c => Some(Message::CopyToClipboard),
            _ if self.delay == c => Some(Message::MenuAction(Modals::DelayTime)),
            _ if self.resize == c => Some(Message::Resize),
            _ if self.screenshot == c => Some(Message::Screenshot),
            _ => None,
        };
    }
    

    pub fn char_already_used(&self, c: char) -> bool {
        if 
        self.save == c || self.saveas == c || self.copy == c || 
        self.delete == c || self.exit == c || self.delay == c ||
        self.resize == c || self.screenshot == c {
            return true;
        }
        return false;
    }

    pub fn assign_new_value(&mut self, new_char: char, hotkey: HotkeysMap) {
        match hotkey {
            HotkeysMap::Save => self.set_save(new_char),
            HotkeysMap::SaveAs => self.set_saveas(new_char),
            HotkeysMap::Delete => self.set_delete(new_char),
            HotkeysMap::Exit => self.set_exit(new_char),
            HotkeysMap::Copy => self.set_copy(new_char),
            HotkeysMap::Delay => self.set_delay(new_char),
            HotkeysMap::Resize => self.set_resize(new_char),
            HotkeysMap::Screenshot => self.set_screenshot(new_char),
            HotkeysMap::None => (),
        }
    }

    pub fn save_hotkeys(&self) -> Result<(), String> {
        let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
        let new_dir = PathBuf::from(format!("{}/{}", dir.data_local_dir().to_str().ok_or("Error getting data local dir")?, "ezScreenshot"));
        let file_path = new_dir.join("hotkey.config");
    
        if !new_dir.exists() {
            fs::create_dir_all(&new_dir).map_err(|err| format!("Error creating directory: {}", err))?;
        } 
        // File already exists, so save the file
        let mut file = File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
        let serialized = serde_json::to_string(self).map_err(|err| format!("Serialization error: {}", err))?;
        file.write_all(serialized.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;
    
        Ok(())
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
            HotkeysMap::Delay => write!(f, "Delay"),
            HotkeysMap::Resize => write!(f, "Resize"),
            HotkeysMap::Screenshot => write!(f, "Screenshot"),
            HotkeysMap::None => write!(f, "None"),
        }
    }
}

