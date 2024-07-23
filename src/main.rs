use std::vec;

use console::show_menu;

mod server;
mod client;
mod console;
pub mod command;

#[tokio::main]
async fn main() {
    let option = show_menu(vec!["Client", "Server"]);
    match option {
        1 => client::run().await,
        2 => server::run().await,
        _ => panic!("Invalid option")
    }
}
