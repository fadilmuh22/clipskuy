// use iced::widget::scrollable::{Scrollbar, Scroller};
// use iced::widget::{button, container, progress_bar, scrollable, text, text_input};
// use iced::{application, theme, Color};

// macro_rules! color {
//     ($red:expr, $green:expr, $blue:expr) => {
//         Color::from_rgba(
//             $red as f32 / 255.0,
//             $green as f32 / 255.0,
//             $blue as f32 / 255.0,
//             1.0,
//         )
//     };
//     ($red:expr, $green:expr, $blue:expr, $opacity:expr) => {
//         Color::from_rgba(
//             $red as f32 / 255.0,
//             $green as f32 / 255.0,
//             $blue as f32 / 255.0,
//             $opacity as f32 / 255.0,
//         )
//     };
// }

// #[derive(Debug, Clone, Copy)]
// pub struct Theme {
//     text: Color,
//     svg: Color,
//     background: Color,
//     foreground: Color,
//     border: Color,

//     primary: Color,
//     secondary: Color,

//     success: Color,
//     warning: Color,
//     error: Color,
// }

// impl Theme {
//     pub const NORMAL: Self = Self {
//         text: Color::BLACK,
//         svg: Color::BLACK,
//         background: color!(255, 255, 255),
//         foreground: color!(248, 248, 242),
//         border: color!(250, 85, 134),

//         primary: color!(189, 147, 249),
//         secondary: color!(46, 144, 255),

//         success: color!(80, 250, 123),
//         warning: color!(241, 250, 140),
//         error: color!(255, 85, 85),
//     };
// }

// impl Default for Theme {
//     fn default() -> Self {
//         Self::NORMAL
//     }
// }

// impl application::StyleSheet for Theme {
//     type Style = ();

//     fn appearance(&self, _style: &Self::Style) -> application::Appearance {
//         application::Appearance {
//             background_color: self.background,
//             text_color: self.text,
//         }
//     }
// }

// impl text::StyleSheet for Theme {
//     type Style = ();

//     fn appearance(&self, _style: Self::Style) -> text::Appearance {
//         text::Appearance {
//             color: self.text.into(),
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, Default)]
// pub enum Container {
//     #[default]
//     Default,
//     Bordered,
// }

// impl container::StyleSheet for Theme {
//     type Style = Container;

//     fn appearance(&self, style: &Self::Style) -> container::Appearance {
//         match style {
//             Container::Default => container::Appearance::default(),
//             Container::Bordered => container::Appearance {
//                 border_color: color!(0x45, 0x85, 0x88),
//                 border_width: 1.0,
//                 border_radius: 4.0,
//                 ..Default::default()
//             },
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, Default)]
// pub enum Button {
//     #[default]
//     Primary,
//     Secondary,
//     Danger,
//     Container,
// }

// impl button::StyleSheet for Theme {
//     type Style = Button;

//     fn active(&self, style: &Self::Style) -> button::Appearance {
//         match style {
//             Button::Primary => button::Appearance {
//                 background: self.primary.into(),
//                 border_radius: 4.0,
//                 border_width: 1.0,
//                 border_color: color!(0x45, 0x85, 0x88),
//                 ..Default::default()
//             },
//             Button::Secondary => button::Appearance {
//                 background: self.secondary.into(),
//                 border_radius: 4.0,
//                 border_width: 1.0,
//                 ..Default::default()
//             },
//             Button::Danger => button::Appearance {
//                 background: self.error.into(),
//                 border_radius: 4.0,
//                 border_width: 1.0,
//                 ..Default::default()
//             },
//             Button::Container => button::Appearance {
//                 background: Color::TRANSPARENT.into(),
//                 ..Default::default()
//             },
//         }
//     }
// }

// impl scrollable::StyleSheet for Theme {
//     type Style = theme::Theme;

//     fn active(&self, style: &Self::Style) -> Scrollbar {
//         style.active(&theme::Scrollable::Default)
//     }

//     fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
//         style.hovered(&theme::Scrollable::Default, is_mouse_over_scrollbar)
//     }

//     fn hovered_horizontal(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
//         if is_mouse_over_scrollbar {
//             Scrollbar {
//                 background: style.active(&theme::Scrollable::default()).background,
//                 border_radius: 0.0,
//                 border_width: 0.0,
//                 border_color: Default::default(),
//                 scroller: Scroller {
//                     color: self.border,
//                     border_radius: 0.0,
//                     border_width: 0.0,
//                     border_color: Default::default(),
//                 },
//             }
//         } else {
//             self.active(style)
//         }
//     }
// }

// impl progress_bar::StyleSheet for Theme {
//     type Style = theme::Theme;

//     fn appearance(&self, style: &Self::Style) -> progress_bar::Appearance {
//         progress_bar::Appearance {
//             background: style.extended_palette().background.strong.color.into(),
//             bar: self.border.into(),
//             border_radius: 0.0,
//         }
//     }
// }

// /// The style of a text input.
// #[derive(Default)]
// pub enum TextInput {
//     /// The default style.
//     #[default]
//     Default,
//     /// A custom style.
//     Custom(Box<dyn text_input::StyleSheet<Style = Theme>>),
// }

// impl text_input::StyleSheet for Theme {
//     type Style = TextInput;

//     fn active(&self, style: &Self::Style) -> text_input::Appearance {
//         if let TextInput::Custom(custom) = style {
//             return custom.active(self);
//         }

//         let palette = self.extended_palette();

//         text_input::Appearance {
//             background: palette.background.base.color.into(),
//             border_radius: 2.0,
//             border_width: 1.0,
//             border_color: palette.background.strong.color,
//             icon_color: palette.background.weak.text,
//         }
//     }

//     fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
//         if let TextInput::Custom(custom) = style {
//             return custom.hovered(self);
//         }

//         let palette = self.extended_palette();

//         text_input::Appearance {
//             background: palette.background.base.color.into(),
//             border_radius: 2.0,
//             border_width: 1.0,
//             border_color: palette.background.base.text,
//             icon_color: palette.background.weak.text,
//         }
//     }

//     fn focused(&self, style: &Self::Style) -> text_input::Appearance {
//         if let TextInput::Custom(custom) = style {
//             return custom.focused(self);
//         }

//         let palette = self.extended_palette();

//         text_input::Appearance {
//             background: palette.background.base.color.into(),
//             border_radius: 2.0,
//             border_width: 1.0,
//             border_color: palette.primary.strong.color,
//             icon_color: palette.background.weak.text,
//         }
//     }

//     fn placeholder_color(&self, style: &Self::Style) -> Color {
//         if let TextInput::Custom(custom) = style {
//             return custom.placeholder_color(self);
//         }

//         let palette = self.extended_palette();

//         palette.background.strong.color
//     }

//     fn value_color(&self, style: &Self::Style) -> Color {
//         if let TextInput::Custom(custom) = style {
//             return custom.value_color(self);
//         }

//         let palette = self.extended_palette();

//         palette.background.base.text
//     }

//     fn selection_color(&self, style: &Self::Style) -> Color {
//         if let TextInput::Custom(custom) = style {
//             return custom.selection_color(self);
//         }

//         let palette = self.extended_palette();

//         palette.primary.weak.color
//     }

//     fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
//         if let TextInput::Custom(custom) = style {
//             return custom.disabled(self);
//         }

//         let palette = self.extended_palette();

//         text_input::Appearance {
//             background: palette.background.weak.color.into(),
//             border_radius: 2.0,
//             border_width: 1.0,
//             border_color: palette.background.strong.color,
//             icon_color: palette.background.strong.color,
//         }
//     }

//     fn disabled_color(&self, style: &Self::Style) -> Color {
//         if let TextInput::Custom(custom) = style {
//             return custom.disabled_color(self);
//         }

//         self.placeholder_color(style)
//     }
// }

use iced::{widget::button, Color};

pub struct ContainerButton;

impl button::StyleSheet for ContainerButton {
    type Style = iced::theme::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Color::TRANSPARENT.into(),
            ..Default::default()
        }
    }
}
