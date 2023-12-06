#[derive(Debug, Clone)]
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

    //TODO char to message converter
}
