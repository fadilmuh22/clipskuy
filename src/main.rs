mod theme;
mod widget;

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{window, Alignment, Application, Command, Length, Padding, Settings};
use serde::Deserialize;

use theme::Theme;
use widget::Element;

pub fn main() -> iced::Result {
    ClipEvent::run(Settings {
        window: window::Settings {
            max_size: Some((400, 400)),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
enum ClipEvent {
    Loading,
    Loaded { clips: Vec<Clipboard> },
    Errored,
}

#[derive(Debug, Clone)]
enum Message {
    ClipboardFetched(Result<Vec<Clipboard>, Error>),
    Search,
}

impl Application for ClipEvent {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ClipEvent, Command<Message>) {
        (
            ClipEvent::Loading,
            Command::perform(Clipboard::fetch_clips(), Message::ClipboardFetched),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            ClipEvent::Loading => String::from("Loading"),
            ClipEvent::Loaded { clips, .. } => String::from(clips.len().to_string()),
            ClipEvent::Errored { .. } => String::from("Whoops!"),
        };

        format!("{subtitle} - ClipSkuy")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ClipboardFetched(Ok(clips)) => {
                *self = ClipEvent::Loaded { clips };

                Command::none()
            }
            Message::ClipboardFetched(Err(_error)) => {
                *self = ClipEvent::Errored;

                Command::none()
            }
            Message::Search => match self {
                ClipEvent::Loading => Command::none(),
                _ => {
                    *self = ClipEvent::Loading;

                    Command::perform(Clipboard::fetch_clips(), Message::ClipboardFetched)
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self {
            ClipEvent::Loading => {
                column![text("Fetching clipboard from server").size(40),].width(Length::Shrink)
            }
            ClipEvent::Loaded { clips } => column![
                row![text("Clipboard").size(24)].padding(16),
                column(clips.iter().map(|clip| clip.view()).collect())
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
            ]
            .into(),

            ClipEvent::Errored => column![
                text("Whoops! Something went wrong...").size(40),
                button("Try again").on_press(Message::Search)
            ]
            .spacing(8)
            .align_items(Alignment::Center),
        };

        container(scrollable(
            container(content).width(Length::Fill).center_x(),
        ))
        .into()
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Clipboard {
    id: String,
    content: String,
    timestamp: String,
}

impl Clipboard {
    fn view(&self) -> Element<Message> {
        row![
            row![text(&self.content[..10]).size(20),]
                .width(Length::Fill)
                .align_items(Alignment::Start),
            row![
                button("Star")
                    .on_press(Message::Search)
                    .style(theme::Button::Primary),
                button("Delete")
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

    async fn fetch_clips() -> Result<Vec<Clipboard>, Error> {
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
            })
            .collect();

        Ok(clips)
    }
}

#[derive(Debug, Clone)]
enum Error {
    APIError,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}
