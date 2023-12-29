    use iced::widget::{button, container, text};
    use iced::{application, color};
    use iced::theme::Container;
    use iced_native::widget::scrollable::style;

    #[derive(Debug, Clone, Copy, Default)]
    pub struct CustomContainer {
        style: ContainerC
    }

    impl CustomContainer {
        pub fn new(style: ContainerC) -> Self {Self {style}}
    }


    #[derive(Debug, Clone, Copy, Default)]
    pub enum ContainerC {
        #[default]
        Default,
        Bordered,
    }

    impl container::StyleSheet for CustomContainer {
        type Style = CustomContainer;

        fn appearance(&self, _: &Self::Style) -> container::Appearance {
            match self.style {
                ContainerC::Default => container::Appearance::default(),
                ContainerC::Bordered => container::Appearance {
                    border_color: color!(0x45, 0x85, 0x88),
                    border_width: 1.0,
                    border_radius: 4.0.into(),
                    ..Default::default()
                },
            }
        }
    }