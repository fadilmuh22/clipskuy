use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::widget::{button, container, progress_bar, scrollable, text};
use iced::{application, theme, Color};

macro_rules! color {
    ($red:expr, $green:expr, $blue:expr) => {
        Color::from_rgba(
            $red as f32 / 255.0,
            $green as f32 / 255.0,
            $blue as f32 / 255.0,
            1.0,
        )
    };
    ($red:expr, $green:expr, $blue:expr, $opacity:expr) => {
        Color::from_rgba(
            $red as f32 / 255.0,
            $green as f32 / 255.0,
            $blue as f32 / 255.0,
            $opacity as f32 / 255.0,
        )
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    text: Color,
    svg: Color,
    background: Color,
    foreground: Color,
    border: Color,

    primary: Color,
    secondary: Color,

    success: Color,
    warning: Color,
    error: Color,
}

impl Theme {
    pub const NORMAL: Self = Self {
        text: Color::BLACK,
        svg: Color::BLACK,
        background: color!(255, 255, 255),
        foreground: color!(248, 248, 242),
        border: color!(250, 85, 134),

        primary: color!(189, 147, 249),
        secondary: color!(46, 144, 255),

        success: color!(80, 250, 123),
        warning: color!(241, 250, 140),
        error: color!(255, 85, 85),
    };
}

impl Default for Theme {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.background,
            text_color: self.text,
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: self.text.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
    Bordered,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Default => container::Appearance::default(),
            Container::Bordered => container::Appearance {
                border_color: color!(0x45, 0x85, 0x88),
                border_width: 1.0,
                border_radius: 4.0,
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Primary,
    Secondary,
    Danger,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: self.primary.into(),
                border_radius: 4.0,
                border_width: 1.0,
                border_color: color!(0x45, 0x85, 0x88),
                ..Default::default()
            },
            Button::Secondary => button::Appearance {
                background: self.secondary.into(),
                ..Default::default()
            },
            Button::Danger => button::Appearance {
                background: self.error.into(),
                ..Default::default()
            },
        }
    }
}

impl scrollable::StyleSheet for Theme {
    type Style = theme::Theme;

    fn active(&self, style: &Self::Style) -> Scrollbar {
        style.active(&theme::Scrollable::Default)
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        style.hovered(&theme::Scrollable::Default, is_mouse_over_scrollbar)
    }

    fn hovered_horizontal(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        if is_mouse_over_scrollbar {
            Scrollbar {
                background: style.active(&theme::Scrollable::default()).background,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                scroller: Scroller {
                    color: self.border,
                    border_radius: 0.0,
                    border_width: 0.0,
                    border_color: Default::default(),
                },
            }
        } else {
            self.active(style)
        }
    }
}

impl progress_bar::StyleSheet for Theme {
    type Style = theme::Theme;

    fn appearance(&self, style: &Self::Style) -> progress_bar::Appearance {
        progress_bar::Appearance {
            background: style.extended_palette().background.strong.color.into(),
            bar: self.border.into(),
            border_radius: 0.0,
        }
    }
}
