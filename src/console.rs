use std::io;
use std::io::Write;
use std::process::exit;
use colored::Colorize;
use crate::config::S3Config;
use crate::crypto::{decrypt_config, encrypt_config, gen_new_keys, new_keys, random_mnemonic, test_crypto};
use crate::s3::S3Client;

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
    println!("{}","  ls - list buckets/folders".green());
    println!("{}","  ls <bucket/folder name> - list files/objects in specified <bucket/folder name>".green());
    println!("{}","  mkdir <bucket/folder name> - creates new bucket/folder".green());
    println!("{}","  config - prints used config".green());
    println!("{}","  keys - generates new crypto keys !danger! - rewrites existing keys".green());
    println!("{}","  q (exit/quit) - to exit this app".green());
    println!();
}

pub(crate) fn y_or_n(question: &str) -> bool {

    let stdin = io::stdin();
    let input = &mut String::new();

    input.clear();

    print!("{}\n{}", question, " (y/n) > ".red());
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

pub(crate) fn ask(question: &str) -> String {
    let stdin = io::stdin();
    let input = &mut String::new();

    input.clear();

    print!("{} : ",question.red());
    io::stdout().flush().expect("error flashing terminal");

    stdin.read_line(input).expect("error reading user input");
    let input = String::from(input.trim());

    return input
}

pub(crate) async fn console_loop() {
    let conf: S3Config = S3Config::init();
    let s3cli = S3Client{ config: conf.clone() };
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        input.clear();

        print!("{}", " s3po > ".red());
        io::stdout().flush().expect("error flashing terminal");

        stdin.read_line(input).expect("error reading user input");
        let input = &mut String::from(input.trim());

        if input == "help" {
            print_help();
            continue
        }

        if input == "encrypt" {
            encrypt_config(conf.clone());
            //println!("{}", "must encrypt something".blue());
            continue
        }

        if input == "decrypt" {
            decrypt_config(conf.clone());
            //println!("{}", "must decrypt something".blue());
            continue
        }

        if input == "keys" {
            gen_new_keys(conf.clone());
            continue
        }

        if input == "todo" {
            print_todo();
            continue
        }

        if input == "mnemonic" {
            println!("{}", random_mnemonic());
            continue
        }

        if input.starts_with("mkdir") {
            let input_vec: Vec<_>  = input.split(" ").collect();
            if input_vec.len() > 1 {
                let bucket_name = input_vec[1].to_string();
                s3cli.mkdir(bucket_name).await;
            } else {
                let bucket_name = ask("Enter new bucket name");
                s3cli.mkdir(bucket_name).await;
            }
            continue
        }

        if input.starts_with("ls") || input.starts_with("list") {
            let input_vec: Vec<_>  = input.split(" ").collect();
            if input_vec.len() > 1 {
                s3cli.ls(input_vec[1].to_string()).await;
                continue
            }
            s3cli.ls("".to_string()).await;
            continue
        }

        if input == "put" {
            println!("{}", "must put/upload object to specified bucket".blue());
            continue
        }

        if input == "get" {
            println!("{}", "must get/download object from specified bucket".blue());
            continue
        }

        if input == "config" {
            println!("{:?}", conf);
            continue
        }

        if input == "q" || input == "exit" || input == "quit" {
            println!("{}", "buy...".yellow());
            exit(0);
        }

        //tests

        if input == "test_crypto" {
            test_crypto();
            continue
        }

        println!("your input: {} - is not a command...", input);
        print_help();
    }
}