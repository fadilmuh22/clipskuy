use once_cell::sync::Lazy;

use iced::alignment::Alignment;
use iced::event::{self, Event};
use iced::widget::{
    button, column, container, focus_next, focus_previous, row, scrollable, text, text_input,
};
use iced::{keyboard, subscription, Application, Command, Element, Length, Subscription};

use crate::types::{Error, Filter};
use crate::widgets::clip_detail::ClipDetail;
use crate::widgets::clip_item::ClipItem;

#[derive(Debug)]
pub enum ClipSkuy {
    Loading,
    Loaded { clip_item_list: Vec<ClipItem> },
    Detail { clip_detail: ClipDetail },
    Errored,
}

#[derive(Debug, Default)]
struct State {
    input_value: String,
    filter: Filter,
    tasks: Vec<ClipItem>,
    dirty: bool,
    saving: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<Vec<ClipItem>, Error>),
    DetailNavigate { clip_detail: ClipDetail },
    InputChanged(String),
    Search,
    TabPressed { shift: bool },
}

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

impl Application for ClipSkuy {
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ClipSkuy, Command<Message>) {
        (
            ClipSkuy::Loading,
            Command::perform(ClipItem::fetch_clips(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            ClipSkuy::Loading => String::from("Loading"),
            ClipSkuy::Loaded {
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
        match self {
            ClipSkuy::Loading => {
                match message {
                    Message::Loaded(Ok(clip_item_list)) => {
                        *self = ClipSkuy::Loaded { clip_item_list };
                    }
                    Message::Loaded(Err(_)) => {
                        *self = ClipSkuy::Errored;
                    }
                    _ => {}
                };

                text_input::focus(INPUT_ID.clone())
            }
            ClipSkuy::Loaded { clip_item_list } => {
                let command = match message {
                    Message::Loaded(Ok(clips)) => {
                        *self = ClipSkuy::Loaded {
                            clip_item_list: clips,
                        };

                        Command::none()
                    }
                    Message::Loaded(Err(_error)) => {
                        *self = ClipSkuy::Errored;

                        Command::none()
                    }
                    Message::InputChanged(_) => todo!(),
                    Message::Search => match self {
                        ClipSkuy::Loading => Command::none(),
                        _ => {
                            *self = ClipSkuy::Loading;

                            Command::perform(ClipItem::fetch_clips(), Message::Loaded)
                        }
                    },
                    Message::DetailNavigate { clip_detail } => {
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
                };

                Command::batch(vec![command])
            }
            ClipSkuy::Detail { clip_detail } => todo!(),
            ClipSkuy::Errored => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self {
            ClipSkuy::Loading => {
                column![text("Fetching clipboard from server").size(40),].width(Length::Shrink)
            }
            ClipSkuy::Loaded { clip_item_list } => column![
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

        let value = "value";

        let search_input = text_input("Search", value)
            .id(INPUT_ID.clone())
            .on_input(Message::InputChanged)
            .on_submit(Message::Search)
            .padding(15)
            .size(30);

        let wrapper = column![search_input, content];

        container(scrollable(
            container(wrapper).width(Length::Fill).center_x(),
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
