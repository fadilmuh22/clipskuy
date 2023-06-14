mod clipskuy;
mod theme;
mod types;
mod widget_types;
mod widgets;

use dotenv::dotenv;

use iced::Application;

#[tokio::main]
pub async fn main() -> iced::Result {
    dotenv().ok();

    clipskuy::ClipSkuy::run(iced::Settings {
        window: iced::window::Settings {
            max_size: Some((400, 400)),
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    })
}
