use crate::app::{Message, MenuAction};

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

        // Getter methods
        pub fn get_save(&self) -> char {
            self.save
        }
    
        pub fn get_saveas(&self) -> char {
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

    //TODO char to message converter
}

impl Hotkeys {

    pub fn to_message(&self, c: char) -> Option<Message> {
        return match c {
            _ if self.save == c => Some(Message::MenuAction(MenuAction::Save)),
            _ if self.saveas == c => Some(Message::MenuAction(MenuAction::SaveAs)),
            _ if self.delete == c => Some(Message::Drop),
            _ if self.exit == c => None, //TODO: implement close application
            _ if self.copy == c => None, //TODO: copy to clipboard
            _ if self.settings == c => Some(Message::MenuAction(MenuAction::Settings)),
            _ if self.resize == c => Some(Message::Resize),
            _ if self.screenshot == c => Some(Message::Screenshot),
            _ => None,
        };
    }
}


