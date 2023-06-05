use iced::alignment::Alignment;
use iced::widget::{button, column, row, text};
use iced::{Length, Padding};
use serde::Deserialize;

use super::icons::{delete_icon, edit_icon, star_empty_icon, star_fill_icon};
use crate::theme;
use crate::types::Clip;
use crate::{Element, Message};

#[derive(Debug, Clone, Deserialize)]
pub struct ClipDetail {
    pub clip: Clip,
}

impl ClipDetail {
    pub fn view(&self) -> Element<Message> {
        column![
            row![
                row![
                    button(edit_icon()).on_press(Message::Search),
                    text("Detail").size(24)
                ]
                .spacing(8)
                .width(Length::Fill),
                row![
                    button(match self.clip.starred {
                        true => star_fill_icon(),
                        false => star_empty_icon(),
                    })
                    .on_press(Message::Search)
                    .style(theme::Button::Primary),
                    button(delete_icon())
                        .on_press(Message::Search)
                        .style(theme::Button::Danger),
                ]
                .spacing(8)
                .align_items(Alignment::End)
            ],
            row![text(&self.clip.content).size(20)].align_items(Alignment::Start),
        ]
        .padding(32)
        .spacing(16)
        .width(Length::Fill)
        .into()
    }
}
