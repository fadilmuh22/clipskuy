use iced::alignment::Alignment;
use iced::theme::Button;
use iced::widget::{button, row, text};
use iced::{Element, Length, Padding};
use serde::Deserialize;

use super::clip_detail::ClipDetail;
use super::icons::{delete_icon, star_empty_icon, star_fill_icon};
use crate::clipskuy::Message;
use crate::themes::theme::ContainerButton;
use crate::types::{Clip, Error};

#[derive(Debug, Clone, Deserialize)]
pub struct ClipItem {
    clip: Clip,
}

impl From<Clip> for ClipItem {
    fn from(clip: Clip) -> Self {
        Self { clip: clip }
    }
}

impl ClipItem {
    pub fn view(&self) -> Element<Message> {
        button(
            row![
                row![text(&self.clip.content[..10]).size(20),]
                    .width(Length::Fill)
                    .align_items(Alignment::Start),
                row![
                    button(match self.clip.starred {
                        true => star_fill_icon(),
                        false => star_empty_icon(),
                    })
                    .on_press(Message::Search)
                    .style(Button::Primary),
                    button(delete_icon())
                        .on_press(Message::Search)
                        .style(Button::Destructive),
                ]
                .align_items(Alignment::End)
                .spacing(8)
            ]
            .width(Length::Fill)
            .padding(Padding::from([0, 16, 16, 16])),
        )
        .style(Button::Custom(Box::new(ContainerButton)))
        .on_press(Message::DetailNavigate {
            clip_detail: ClipDetail {
                clip: self.clip.clone(),
            },
        })
        .into()
    }

    pub async fn fetch_clips() -> Result<Vec<ClipItem>, Error> {
        #[derive(Deserialize)]
        struct Todo {
            id: u32,
            title: String,
            completed: bool,
        }

        let fetch_entry = async {
            let url = format!("https://jsonplaceholder.typicode.com/todos?_page=0&_limit=10");

            reqwest::get(&url).await?.json().await
        };

        let todos: Vec<Todo> = fetch_entry.await?;

        let clip_item_list = todos
            .iter()
            .map(|todo| ClipItem {
                clip: Clip {
                    id: todo.id.to_string(),
                    content: todo.title.to_string(),
                    timestamp: todo.completed.to_string(),
                    starred: todo.completed,
                },
            })
            .collect::<Vec<ClipItem>>();

        Ok(clip_item_list)
    }
}
