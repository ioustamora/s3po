mod console;
mod config;
mod s3;
mod crypto;
mod command;

use crate::command::parse_args;
use crate::console::{print_intro, console_loop};

#[::tokio::main]
async fn main() {
    print_intro();
    parse_args().await;
    console_loop().await;
}