use iced::alignment::Alignment;
use iced::theme::Button;
use iced::widget::{button, column, row, text};
use iced::{Element, Length};
use serde::Deserialize;

use super::icons::{back_icon, delete_icon, star_empty_icon, star_fill_icon};
use crate::clipskuy::Message;
use crate::types::Clip;

#[derive(Debug, Clone)]
pub enum ClipState {
    Idle,
    Editing,
}

impl Default for ClipState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClipDetail {
    pub clip: Clip,
}

impl ClipDetail {
    pub fn view(&self) -> Element<Message> {
        column![
            row![
                row![
                    button(back_icon()).on_press(Message::Search),
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
                    .style(Button::Primary),
                    button(delete_icon())
                        .on_press(Message::Search)
                        .style(Button::Primary),
                ]
                .spacing(8)
                .align_items(Alignment::End)
            ],
            row![text(&self.clip.content).size(20),].align_items(Alignment::Start),
        ]
        .padding(32)
        .spacing(16)
        .width(Length::Fill)
        .into()
    }
}
