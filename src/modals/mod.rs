pub mod hotkeys_modal;
pub mod save_as_modal;
pub mod settings_modal;
pub mod setdefaultpath_modal;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Modals {
    #[default]
    SaveAs,
    Save,
    Settings,
    Hotkeys,
    SetPath,
    None
}