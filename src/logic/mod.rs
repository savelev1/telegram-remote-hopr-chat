use std::path::Path;

use telegram_bot::{Message, MessageKind};

use crate::process::thread_command::{ThreadCommand, ThreadCommandAction};
use crate::synchronizer::Synchronizer;

const HELP: &str = "
<b>Hopr Chat commands</b>
/start_hopr - start the Hopr Chat
/crawl - crawls the network and tries to find other nodes
/ping xHOPR_address - pings another node to check its availability
/ping_randobot - if RandoBot answers you, it means other nodes can reach you! Required <code>/includeRecipient y</code>
/ping_coverbot - required <code>/includeRecipient y</code>
/my_address - shows the address of this node
/balance - shows our current native and hopr balance
/send xHOPR_address message - sends a message to another party
/send_to_author message - sends a message to bot author.
/includeRecipient [y | n] - set includeRecipient (prepends your address to all messages)
/quit - send /stdin quit
<b>For other commands</b> send <code>/stdin command</code>, e.g.: <code>/stdin help</code>

<b>Common commands</b>
/run - run a process. e.g.: <code>/run ls</code>, <code>/run ping google.com</code>, etc.
/kill - stop the running process
/stdin text - send stdin \"text\" in the running process
/help - show help
";

pub async fn on_bot_message(synchronizer: &mut Synchronizer, message: &Message) {
    if let MessageKind::Text { ref data, .. } = message.kind {
        let message_from_id = message.from.id.to_string().parse::<u64>().unwrap();
        if synchronizer.config.telegram_user_ids.len() == 0 || synchronizer.config.telegram_user_ids.contains(&message_from_id) {
            if synchronizer.bot.is_started {
                if data == "/start" {
                    synchronizer.bot.send(&message, &String::from("Already started")).await;
                } else if data.starts_with("/start_hopr") {
                    on_start_hopr_command(synchronizer, &message).await
                } else if data.starts_with("/crawl") {
                    if synchronizer.is_running_process() {
                        synchronizer.send_stdin_to_process(String::from("crawl"));
                    }
                } else if data.starts_with("/ping_randobot") {
                    if synchronizer.is_running_process() {
                        synchronizer.send_stdin_to_process(format!("ping {}", synchronizer.config.randobot_xhopr_address));
                    }
                } else if data.starts_with("/ping_coverbot") {
                    if synchronizer.is_running_process() {
                        synchronizer.send_stdin_to_process(format!("ping {}", synchronizer.config.coverbot_xhopr_address));
                    }
                } else if data.starts_with("/ping") {
                    if synchronizer.is_running_process() {
                        let parts = data.split_whitespace().collect::<Vec<&str>>();
                        if parts.len() == 2 {
                            synchronizer.send_stdin_to_process(format!("ping {}", parts[1]));
                        }
                    }
                } else if data.starts_with("/my_address") {
                    if synchronizer.is_running_process() {
                        synchronizer.send_stdin_to_process(String::from("myAddress"));
                    }
                } else if data.starts_with("/balance") {
                    if synchronizer.is_running_process() {
                        synchronizer.send_stdin_to_process(String::from("balance"));
                    }
                } else if data.starts_with("/send_to_author") {
                    if synchronizer.is_running_process() {
                        let parts = data.split_whitespace().collect::<Vec<&str>>();
                        if parts.len() == 2 {
                            synchronizer.send_stdin_to_process(format!("send 16Uiu2HAm7hqva9iw7tbTS5q8bCP9eSrRYpqYgPHPRRrgKtZwiEts"));
                            synchronizer.send_stdin_to_process(format!("{}", parts[1]));
                        }
                    }
                } else if data.starts_with("/send") {
                    if synchronizer.is_running_process() {
                        let parts = data.split_whitespace().collect::<Vec<&str>>();
                        if parts.len() == 3 {
                            synchronizer.send_stdin_to_process(format!("send {}", parts[1]));
                            synchronizer.send_stdin_to_process(format!("{}", parts[2]));
                        }
                    }
                } else if data.starts_with("/includeRecipient") {
                    if synchronizer.is_running_process() {
                        let parts = data.split_whitespace().collect::<Vec<&str>>();
                        if parts.len() == 2 {
                            synchronizer.send_stdin_to_process(String::from("includeRecipient"));
                            synchronizer.send_stdin_to_process(format!("{}", parts[1]));
                        }
                    }
                } else if data.starts_with("/quit") {
                    if synchronizer.is_running_process() {
                        synchronizer.send_stdin_to_process(String::from("quit"));
                    }
                } else if data.starts_with("/run") {
                    on_run_command(synchronizer, &message, data).await
                } else if data.starts_with("/kill") {
                    on_kill_command(synchronizer, &message).await
                } else if data.starts_with("/stdin") {
                    on_stdin_command(synchronizer, data);
                } else if data == "/help" {
                    synchronizer.bot.send(&message, &HELP.to_string()).await;
                } else {
                    synchronizer.bot.send(&message, &format!("I don't know a <code>{}</code> command. Type <code>/help</code>.", data)).await;
                }
            } else {
                if data == "/start" {
                    synchronizer.bot.is_started = true;
                    synchronizer.bot.send(&message, &format!("Hello {}!\nYour id: <code>{}</code>\nType <code>/help</code> for more info.\nType <code>/start_hopr</code> to start the Hopr Chat.", message.from.first_name, message.from.id)).await;
                } else {
                    synchronizer.bot.send(&message, &String::from("Please type <code>/start</code>.")).await;
                }
            }
        } else {
            synchronizer.bot.send(&message, &format!("Sorry, I do not talk to strangers. Your id <code>{}</code>.", message.from.id)).await;
        }
    }
}

fn on_stdin_command(synchronizer: &mut Synchronizer, data: &String) {
    let text = data.replace("/stdin", "").trim().to_owned();
    synchronizer.send_stdin_to_process(text);
}

async fn on_kill_command(synchronizer: &mut Synchronizer, message: &&Message) {
    if synchronizer.is_running_process() {
        synchronizer.bot.send(&message, &format!("Killed \"{}\"", synchronizer.running_command.clone())).await;
        synchronizer.terminate_process();
    } else {
        synchronizer.bot.send(&message, &String::from("No running process.")).await;
    }
}

async fn on_run_command(synchronizer: &mut Synchronizer, message: &&Message, data: &String) {
    if synchronizer.is_running_process() {
        synchronizer.bot.send(&message, &format!("Already running \"{}\"", synchronizer.running_command)).await;
    } else {
        let command = data.replace("/run", "").trim().to_owned();
        synchronizer.bot.send(&message, &format!("Running... \"{}\"", command)).await;
        synchronizer.run_process(&command);
    }
}

async fn on_start_hopr_command(synchronizer: &mut Synchronizer, message: &&Message) {
    if synchronizer.is_running_process() {
        synchronizer.bot.send(&message, &format!("Already running \"{}\"", synchronizer.running_command)).await;
    } else {
        let mut dir = format!("{}", &synchronizer.config.hopr_chat_directory);
        if dir.ends_with('/') { dir.pop(); }
        let start_hopr_chat_template = "start-hopr-chat";
        let read_dir = Path::new(&dir).read_dir().unwrap();
        let mut start_hopr_chat = String::new();
        for dir_entry in read_dir {
            let dir_entry = dir_entry.unwrap();
            let file_name_string = dir_entry.file_name().into_string().unwrap();
            if dir_entry.path().is_file() && file_name_string.contains(start_hopr_chat_template) {
                start_hopr_chat = file_name_string;
            }
        }
        if start_hopr_chat.len() > 0 {
            let command = Path::new(&format!("{}/{}", dir, start_hopr_chat)).to_str().unwrap().to_owned();
            synchronizer.bot.send(&message, &format!("Running... \"{}\"", command)).await;
            synchronizer.run_process(&command);
        } else {
            synchronizer.bot.send(&message, &format!("File {}.* not found in {}.", start_hopr_chat_template, dir)).await;
        }
    }
}

pub async fn on_process_message(synchronizer: &mut Synchronizer, message: &Message, command: ThreadCommand) {
    match command.action {
        ThreadCommandAction::Exit => {
            synchronizer.bot.send_code(&message, &command.data).await;
            synchronizer.set_no_running_process();
        }
        ThreadCommandAction::Error => {
            synchronizer.bot.send_pre(&message, &command.data).await;
            synchronizer.terminate_process();
        }
        ThreadCommandAction::Stdout => {
            synchronizer.bot.send_pre(&message, &command.data).await;
        }
        ThreadCommandAction::Stderr => {
            synchronizer.bot.send_code(&message, &command.data).await;
        }
        _ => {}
    }
}