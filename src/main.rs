mod console;
mod config;
mod s3;
mod crypto;

use crate::console::{print_intro, console_loop};

#[::tokio::main]
async fn main() {
    print_intro();
    console_loop().await;
}