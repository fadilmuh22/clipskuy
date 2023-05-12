mod theme;
mod widget;
mod widgets;

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{window, Application, Command, Length, Settings};

use widgets::clipboard::{Clipboard, Error};

use iced::alignment::Alignment;

use theme::Theme;
use widget::{Element, Text};

pub fn main() -> iced::Result {
    ClipSkuy::run(Settings {
        window: window::Settings {
            max_size: Some((400, 400)),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
enum ClipSkuy {
    Loading,
    Loaded { clips: Vec<Clipboard> },
    Errored,
}

#[derive(Debug, Clone)]
pub enum Message {
    ClipboardFetched(Result<Vec<Clipboard>, Error>),
    Search,
}

impl Application for ClipSkuy {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ClipSkuy, Command<Message>) {
        (
            ClipSkuy::Loading,
            Command::perform(Clipboard::fetch_clips(), Message::ClipboardFetched),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            ClipSkuy::Loading => String::from("Loading"),
            ClipSkuy::Loaded { clips, .. } => String::from(clips.len().to_string()),
            ClipSkuy::Errored { .. } => String::from("Whoops!"),
        };

        format!("{subtitle} - ClipSkuy")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ClipboardFetched(Ok(clips)) => {
                *self = ClipSkuy::Loaded { clips };

                Command::none()
            }
            Message::ClipboardFetched(Err(_error)) => {
                *self = ClipSkuy::Errored;

                Command::none()
            }
            Message::Search => match self {
                ClipSkuy::Loading => Command::none(),
                _ => {
                    *self = ClipSkuy::Loading;

                    Command::perform(Clipboard::fetch_clips(), Message::ClipboardFetched)
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self {
            ClipSkuy::Loading => {
                column![text("Fetching clipboard from server").size(40),].width(Length::Shrink)
            }
            ClipSkuy::Loaded { clips } => column![
                row![text("Clipboard").size(24)].padding(16),
                column(clips.iter().map(|clip| clip.view()).collect())
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
            ]
            .into(),

            ClipSkuy::Errored => column![
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
