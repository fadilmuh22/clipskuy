mod clipskuy;
mod themes;
mod types;
mod widgets;

use dotenv::dotenv;

use iced::window;
use iced::Application;

#[tokio::main]
pub async fn main() -> iced::Result {
    dotenv().ok();

    clipskuy::ClipSkuy::run(iced::Settings {
        window: window::Settings {
            max_size: Some((400, 400)),
            ..window::Settings::default()
        },
        ..iced::Settings::default()
    })
}
