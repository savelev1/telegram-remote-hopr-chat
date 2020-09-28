use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use telegram_bot::*;

use crate::logic::{on_bot_message, on_process_message};
use crate::process::Process;
use crate::process::thread_command::{ThreadCommand, ThreadCommandAction};
use crate::tg_bot::TgBot;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub telegram_bot_token: String,
    pub telegram_user_ids: Vec<u64>,
    pub hopr_chat_directory: String,
    pub randobot_xhopr_address: String,
    pub coverbot_xhopr_address: String,
}

pub struct Synchronizer
{
    pub config: Config,
    pub bot: TgBot,
    bot_main_receiver: Option<Receiver<Message>>,
    process_main_sender: Option<Sender<ThreadCommand>>,
    process_main_receiver: Option<Receiver<ThreadCommand>>,
    pub running_command: String,
}

impl Synchronizer
{
    pub fn new() -> Synchronizer {
        let file = File::open("config.json").expect("Can't open config.json");
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader).expect("Can't parse JSON");
        let token = config.telegram_bot_token.clone();

        Synchronizer {
            config,
            bot: TgBot::new(&token),
            bot_main_receiver: None,
            process_main_sender: None,
            process_main_receiver: None,
            running_command: String::new(),
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        self.run_bot().await;
        self.run_ticker().await;

        Ok(())
    }

    async fn run_bot(&mut self) {
        let (bot_thread_sender, bot_main_receiver) = mpsc::channel();
        self.bot_main_receiver = Some(bot_main_receiver);
        self.bot.run(bot_thread_sender).await;
    }

    pub fn run_process(&mut self, command: &String) {
        if self.is_running_process() == false {
            self.running_command = command.clone();
            let (process_thread_sender, process_main_receiver) = mpsc::channel();
            let (process_main_sender, process_thread_receiver) = mpsc::channel();
            self.process_main_sender = Some(process_main_sender);
            self.process_main_receiver = Some(process_main_receiver);
            let cloned_process_thread_sender = process_thread_sender.clone();
            let cloned_command = command.clone();
            thread::spawn(move || {
                let mut process = Process::new(
                    &cloned_command,
                    cloned_process_thread_sender,
                    process_thread_receiver,
                );
                process.start();
            });
        }
    }

    async fn run_ticker(&mut self) {
        let mut messages: Vec<Message> = Vec::new();
        loop {
            self.tick(&mut messages).await;
        }
    }

    async fn tick(&mut self, mut messages: &mut Vec<Message>) {
        self.check_bot_messages(&mut messages).await;
        self.check_process_message(&mut messages).await;
    }

    async fn check_bot_messages(&mut self, messages: &mut Vec<Message>) {
        match self.bot_main_receiver.as_ref().unwrap().recv_timeout(Duration::from_millis(50)) {
            Ok(message) => {
                if messages.len() == 0 {
                    messages.push(message);
                } else {
                    messages[0] = message;
                }

                on_bot_message(self, &messages[0]).await;
            }
            Err(_) => (),
        }
    }

    async fn check_process_message(&mut self, messages: &mut Vec<Message>) {
        if self.process_main_receiver.is_some() {
            match self.process_main_receiver.as_ref().unwrap().recv_timeout(Duration::from_millis(50)) {
                Ok(command) => {
                    on_process_message(self, &messages[0], command).await;
                }
                Err(_) => (),
            }
        }
    }

    pub fn is_running_process(&self) -> bool {
        self.running_command.len() > 0
    }

    pub fn terminate_process(&mut self) {
        self.process_main_sender.as_ref().unwrap().send(ThreadCommand::new(ThreadCommandAction::Terminate, String::new())).unwrap();
        self.set_no_running_process();
    }

    pub fn set_no_running_process(&mut self) {
        self.running_command = String::new();
    }

    pub fn send_stdin_to_process(&self, text: String) {
        self.process_main_sender.as_ref().unwrap().send(ThreadCommand::new(ThreadCommandAction::Stdin, text)).unwrap();
    }
}