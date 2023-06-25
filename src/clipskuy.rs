use iced::alignment::Alignment;
use iced::event::{self, Event};
use iced::widget::{button, column, container, focus_next, focus_previous, row, scrollable, text};
use iced::{keyboard, subscription, Application, Command, Length, Subscription};

use crate::themes::theme::Theme;
use crate::themes::types::Element;
use crate::widgets::clip_detail::ClipDetail;
use crate::widgets::clip_item::{ClipItem, Error};

#[derive(Debug)]
pub enum ClipSkuy {
    Loading,
    ListLoaded { clip_item_list: Vec<ClipItem> },
    Detail { clip_detail: ClipDetail },
    Errored,
}

#[derive(Debug, Clone)]
pub enum Message {
    ClipboardFetched(Result<Vec<ClipItem>, Error>),
    ClipDetailNavigate { clip_detail: ClipDetail },
    Search,
    TabPressed { shift: bool },
}

impl Application for ClipSkuy {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ClipSkuy, Command<Message>) {
        (
            ClipSkuy::Loading,
            Command::perform(ClipItem::fetch_clips(), Message::ClipboardFetched),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            ClipSkuy::Loading => String::from("Loading"),
            ClipSkuy::ListLoaded {
                clip_item_list: clips,
                ..
            } => String::from(clips.len().to_string()),
            ClipSkuy::Detail { clip_detail } => String::from(format!(
                "{} - {}",
                clip_detail.clip.id,
                &clip_detail.clip.content[..5]
            )),
            ClipSkuy::Errored { .. } => String::from("Whoops!"),
        };

        format!("{subtitle} - ClipSkuy")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ClipboardFetched(Ok(clips)) => {
                *self = ClipSkuy::ListLoaded {
                    clip_item_list: clips,
                };

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

                    Command::perform(ClipItem::fetch_clips(), Message::ClipboardFetched)
                }
            },
            Message::ClipDetailNavigate { clip_detail } => {
                *self = ClipSkuy::Detail { clip_detail };

                Command::none()
            }
            Message::TabPressed { shift } => {
                if shift {
                    return focus_previous();
                } else {
                    return focus_next();
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self {
            ClipSkuy::Loading => {
                column![text("Fetching clipboard from server").size(40),].width(Length::Shrink)
            }
            ClipSkuy::ListLoaded { clip_item_list } => column![
                row![text("Clipboard").size(24)].padding(16),
                column(clip_item_list.iter().map(|clip| clip.view()).collect())
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
            ClipSkuy::Detail { clip_detail } => {
                column![clip_detail.view()].width(Length::Fill).into()
            }
        };

        container(scrollable(
            container(content).width(Length::Fill).center_x(),
        ))
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| match (event, status) {
            (
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Tab,
                    modifiers,
                    ..
                }),
                event::Status::Ignored,
            ) => Some(Message::TabPressed {
                shift: modifiers.shift(),
            }),
            _ => None,
        })
    }
}
