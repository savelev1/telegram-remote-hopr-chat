use std::sync::mpsc::Sender;
use std::thread;

use futures::StreamExt;
use telegram_bot::*;
use tokio::runtime::Runtime;

pub struct TgBot {
    token: String,
    api: Api,
    pub is_started: bool,
}

impl TgBot {
    pub fn new(token: &String) -> TgBot {
        TgBot {
            token: token.clone(),
            api: Api::new(token),
            is_started: false,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let cloned_sender = sender.clone();
        let token = self.token.clone();
        thread::spawn(move || {
            Runtime::new().unwrap().block_on(async {
                let thread_api = Api::new(token);
                let mut stream = thread_api.stream();
                loop {
                    let update = stream.next().await.unwrap().unwrap();
                    if let UpdateKind::Message(message) = update.kind {
                        cloned_sender.send(message).unwrap();
                    }
                }
            })
        });
    }

    pub async fn send(&self, message: &Message, text: &String) {
        if let Err(_err) = self.api.send(message.chat.text(format!("{}", text)).parse_mode(ParseMode::Html)).await {
            println!("Error: {}", _err);
        }
    }

    pub async fn send_pre(&self, message: &Message, text: &String) {
        self.send(message, &format!("<pre>{}</pre>", text)).await;
    }

    pub async fn send_code(&self, message: &Message, text: &String) {
        self.send(message, &format!("<code>{}</code>", text)).await;
    }
}