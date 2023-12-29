    use iced::widget::{button, container, text};
    use iced::{application, color, Theme};
    use iced::theme::Container;
    use iced_native::widget::scrollable::style;

    #[derive(Debug, Clone, Copy, Default)]
    pub struct CustomContainer {
    }

    impl CustomContainer {
        pub fn new() -> Self { CustomContainer {} }
    }


    impl container::StyleSheet for CustomContainer {
        type Style = Theme;

        fn appearance(&self, _: &Self::Style) -> container::Appearance {
            container::Appearance {
                border_color: color!(0x45, 0x85, 0x88),
                border_width: 1.0,
                border_radius: 4.0.into(),
                ..Default::default()
            }
        }
    }