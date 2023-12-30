    use iced::widget::container;
    use iced::{Background,Theme};
    use iced_aw::style::colors::BLACK;

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