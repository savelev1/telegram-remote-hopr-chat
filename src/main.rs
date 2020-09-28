extern crate telegram_remote_hopr_chat;

use telegram_remote_hopr_chat::synchronizer::Synchronizer;

#[tokio::main]
async fn main() {
    let mut synchronizer: Synchronizer = Synchronizer::new();
    synchronizer.run().await.unwrap();
}