use std::io;
use std::io::Write;
use std::process::exit;
use colored::Colorize;
use crate::config::S3Config;
use crate::s3::test;

pub fn print_todo() {
    println!();
    println!("{}","TODO: ".yellow());
    println!();
    println!("{}","  some thing that must be done...".green());
    println!();
}

pub fn print_intro() {
    println!();
    println!("{}","s3po v0.1.0".red());
    println!();
}

pub fn print_help() {
    println!();
    println!("{}","USAGE: ".yellow());
    println!();
    println!("{}","  help - for see this help".green());
    println!("{}","  q (exit/quit) - to exit this app".green());
    println!();
}

pub fn y_or_n(question: &str) -> bool {

    let stdin = io::stdin();
    let input = &mut String::new();

    input.clear();

    print!("{}\n(y/n) > ", question);
    io::stdout().flush().expect("error flashing terminal");

    stdin.read_line(input).expect("error reading user input");
    let input = &mut String::from(input.trim().to_lowercase());

    if input == &String::from("y") ||
        input == &String::from("yes") ||
        input == &String::from("ok") {
        return true;
    }

    return false;
}

pub fn ask(question: &str) -> String {
    let stdin = io::stdin();
    let input = &mut String::new();

    input.clear();

    print!("{}\n > ", question);
    io::stdout().flush().expect("error flashing terminal");

    stdin.read_line(input).expect("error reading user input");
    let input = String::from(input.trim());

    return input
}

pub(crate) async fn console_loop() {
    let conf: S3Config = S3Config::init_config();
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        input.clear();

        print!("> ");
        io::stdout().flush().expect("error flashing terminal");

        stdin.read_line(input).expect("error reading user input");
        let input = &mut String::from(input.trim());

        if input == "help" {
            print_help();
            continue
        }

        if input == "encrypt" {

        }

        if input == "decrypt" {

        }

        if input == "test" {
            test(conf.clone()).await;
            continue
        }

        if input == "q" || input == "exit" || input == "quit" {
            println!("{}", "Buy.".red());
            exit(0);
        }
    }
}