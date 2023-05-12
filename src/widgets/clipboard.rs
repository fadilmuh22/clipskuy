use iced::alignment::Alignment;
use iced::widget::{button, row, text};
use iced::{Length, Padding};
use serde::Deserialize;

use super::icons::{delete_icon, star_empty_icon, star_fill_icon};
use crate::theme;
use crate::{Element, Message};

#[derive(Default)]
struct Component;

#[derive(Debug, Clone, Deserialize)]
pub struct Clipboard {
    id: String,
    content: String,
    timestamp: String,
    starred: bool,
}

impl Clipboard {
    pub fn view(&self) -> Element<Message> {
        row![
            row![text(&self.content[..10]).size(20),]
                .width(Length::Fill)
                .align_items(Alignment::Start),
            row![
                button(match self.starred {
                    true => star_fill_icon(),
                    false => star_empty_icon(),
                })
                .on_press(Message::Search)
                .style(theme::Button::Primary),
                button(delete_icon())
                    .on_press(Message::Search)
                    .style(theme::Button::Danger),
            ]
            .align_items(Alignment::End)
            .spacing(8)
        ]
        .width(Length::Fill)
        .padding(Padding::from([0, 16, 16, 16]))
        .into()
    }

    pub async fn fetch_clips() -> Result<Vec<Clipboard>, Error> {
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

        let clips = todos
            .iter()
            .map(|todo| Clipboard {
                id: todo.id.to_string(),
                content: todo.title.to_string(),
                timestamp: todo.completed.to_string(),
                starred: todo.completed,
            })
            .collect();

        Ok(clips)
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    APIError,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}
