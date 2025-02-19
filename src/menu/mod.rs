use iced::widget::{column, Column};
use iced::widget::{
    button, container, horizontal_space, row, text,
};
use iced::{alignment, theme, Color, Element, Length};

use iced_aw::menu::{menu_tree::MenuTree, CloseCondition, ItemHeight, ItemWidth, PathHighlight};
use iced_aw::{helpers::menu_tree, menu_bar, menu_tree};
use crate::app::{App, Message};
use crate::modals::Modals;

struct ButtonStyle;
impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            border_radius: [4.0; 4].into(),
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let plt = style.extended_palette();

        button::Appearance {
            background: Some(Color::from_rgb8(220, 220, 220).into()),
            text_color: plt.primary.weak.text,
            ..self.active(style)
        }
    }

}

fn return_action(label: &str) -> Modals {
    match label.into() {
        "Save" => Modals::Save,
        "Save as..." => Modals::SaveAs,
        "Delay" => Modals::DelayTime,
        "Hotkeys" => Modals::Hotkeys,
        "Default Path" => Modals::SetPath,
        _ => Modals::None
    }
}

fn base_button<'a>(
    content: impl Into<Element<'a, Message, iced::Renderer>>,
    msg: Message,
) -> button::Button<'a, Message, iced::Renderer> {
    button(content)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
        .on_press(msg)
}

fn labeled_button<'a>(label: &str, msg: Message) -> button::Button<'a, Message, iced::Renderer> {
    base_button(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
        msg,
    )
}

fn debug_button<'a>(label: &str) -> button::Button<'a, Message, iced::Renderer> {
    labeled_button(label, Message::MenuAction(return_action(label)))
}

fn debug_item<'a>(label: &str) -> MenuTree<'a, Message, iced::Renderer> {
    menu_tree!(debug_button(label).width(Length::Fill).height(Length::Fill))
}

fn menu_file<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let root = menu_tree(
        container(text("File")
            .style(theme::Text::Color(Color::from_rgb8(255, 255, 255))))
            .padding([0, 2, 0, 2]),
        vec![
            debug_item("Save"),
            debug_item("Save as...")
        ],
    ).width(110);

    root
}

fn menu_settings<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let root = menu_tree(
        container(text("Advanced")
            .style(theme::Text::Color(Color::from_rgb8(255, 255, 255))))
            .padding([0, 2, 0, 2]),
        vec![
            debug_item("Hotkeys"),
            debug_item("Default Path"),
            debug_item("Delay")
        ],
    ).width(110);

    root
}

pub fn top_menu(_app: &App) -> Column<'_, Message> {
    let mb =
        menu_bar!(menu_file(_app), menu_settings(_app))
            .item_width(ItemWidth::Uniform(180))
            .item_height(ItemHeight::Uniform(27))
            .spacing(4.0)
            .bounds_expand(30)
            .main_offset(13)
            .cross_offset(16)
            .path_highlight(Some(PathHighlight::MenuActive))
            .close_condition(CloseCondition {
                leave: true,
                click_outside: false,
                click_inside: false,
            });

    let r = row!(mb, horizontal_space(Length::Fill))
        .padding([2, 8])
        .align_items(alignment::Alignment::Center);

    let top_bar_style: fn(&iced::Theme) -> container::Appearance =
        |_theme| container::Appearance {
            background: Some(Color::from_rgb8(68, 68, 68).into()),
            ..Default::default()
        };
    let top_bar = container(r).width(Length::Fill).style(top_bar_style);


    let c = column![top_bar];

    c
}

