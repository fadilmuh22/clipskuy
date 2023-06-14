use iced::widget::text;
use iced::{alignment, Font};

use crate::widget_types::Text;

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../assets/fonts/icons.ttf"),
};

pub fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(20)
        .horizontal_alignment(alignment::Horizontal::Center)
}

pub fn star_empty_icon() -> Text<'static> {
    icon('\u{E800}')
}

pub fn star_fill_icon() -> Text<'static> {
    icon('\u{E801}')
}

pub fn edit_icon() -> Text<'static> {
    icon('\u{E802}')
}

pub fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}
