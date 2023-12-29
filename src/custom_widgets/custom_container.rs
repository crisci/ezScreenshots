    use iced::widget::{button, container, text};
    use iced::{application, Background, color, Theme};
    use iced::font::Weight::Black;
    use iced::theme::Container;
    use iced_aw::style::colors::BLACK;
    use iced_native::widget::scrollable::style;

    #[derive(Debug, Clone, Copy, Default)]
    pub struct CustomContainer {
        background: Option<Background>
    }


    impl CustomContainer {
        pub fn new(background: Background) -> Self { CustomContainer { background: Some(background) } }
    }


    impl container::StyleSheet for CustomContainer {
        type Style = Theme;

        fn appearance(&self, _: &Self::Style) -> container::Appearance {
            container::Appearance {
                border_color: BLACK,
                border_width: 1.5,
                border_radius: 10.0.into(),
                background: self.background,
                ..Default::default()
            }
        }
    }