use std::io;
use std::io::Write;
use std::process::exit;
use std::string::String;

use colored::Colorize;

use crate::config::S3Config;
use crate::crypto::{decrypt_config, encrypt_config, gen_new_keys, random_mnemonic, test_crypto};
use crate::s3::S3Client;

pub fn print_todo() {
    println!();
    println!("{}","TODO: ".yellow());
    println!();
    println!("{}","  test and write new todos :) ".green());
    println!();
}

pub fn print_intro() {
    println!();
    println!("{}","s3po v0.1.1".red());
    println!();
}

pub fn print_help() {
    println!();
    println!("{}","USAGE: ".yellow());
    println!();
    println!("{}","  help                               - for see this help".green());
    println!("{}","  ls                                 - list buckets".green());
    println!("{}","  ls <bucket name>                   - list files/objects in specified <bucket name>".green());
    println!("{}","  cd <bucket name>                   - change current bucket to specified <bucket name>".green());
    println!("{}","  cd (cd ..)                         - return too root server folder".green());
    println!("{}","  mkdir <bucket name>                - creates new bucket".green());
    println!("{}","  rm <bucket name>                   - delete bucket".green());
    println!("{}","  rm <bucket name> <file name>       - delete file/objects in specified bucket".green());
    println!("{}","  put <bucket name> <file name>      - encrypt and upload <file name> to specified <bucket name>".green());
    println!("{}","  get <bucket name> <file name>      - decrypt and download <file name> from specified <bucket name>".green());
    println!("{}","  upload <bucket name> <file name>   - upload <file name> to specified <bucket name> without encryption".green());
    println!("{}","  download <bucket name> <file name> - download <file name> from specified <bucket name> without decryption".green());
    println!("{}","  config (config print/cat)          - prints used/current/loaded config".green());
    println!("{}","  config list                        - lists all created configs".green());
    println!("{}","  config folder                      - prints path to configs folder".green());
    println!("{}","  config create (add/new)            - creates new config".green());
    println!("{}","  config delete (del/rm) <name>      - delete the config with name".green());
    println!("{}","  config use <name>                  - loads new config and use it to all commands".green());
    println!("{}","  keys                               - generates new crypto keys !danger! - rewrites existing keys".green());
    println!("{}","  q (exit/quit)                      - to exit this app".green());
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
    let mut conf: S3Config = S3Config::init();
    let mut s3cli = S3Client{ config: conf.clone(), bucket: "".to_string() };
    let stdin = io::stdin();
    let input: &mut String = &mut String::new();

    loop {
        input.clear();
        if s3cli.bucket == String::from("") {
            print!("{}", " s3po > ".red());
        } else {
            let bucket_name = s3cli.bucket.clone() + " > ";
            print!("{}{}", " s3po > ".red(), bucket_name.red());
        }

        io::stdout().flush().expect("error flashing terminal");

        stdin.read_line(input).expect("error reading user input");
        let input= &mut String::from(input.trim());

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

        if input.starts_with("cd") {
            let input_vec: Vec<_>  = input.split(" ").collect();
            if input_vec.len() > 1 {
                s3cli.set_bucket(input_vec[1].to_string()).await;
            } else {
                s3cli.set_bucket("".to_string()).await;
            }
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
            } else {
                if s3cli.bucket == "".to_string() {
                    s3cli.ls("".to_string()).await;
                } else {
                    s3cli.ls(s3cli.bucket.clone()).await;
                }
            }
            continue
        }

        if input.starts_with("upload") {
            let input_vec: Vec<_>  = input.split(" ").collect();
            if input_vec.len() > 2 {
                s3cli.put(input_vec[1].to_string(), input_vec[2].to_string(), input_vec[2].to_string()).await;
                continue
            }
            println!("{}", "error putting file... too less args".blue());
            continue
        }
        if input.starts_with("put2") {
            let input_vec: Vec<_>  = input.split(" ").collect();
            if input_vec.len() > 2 {
                s3cli.put2(input_vec[1].to_string(), input_vec[2].to_string(), input_vec[2].to_string()).await;
                continue
            }
            println!("{}", "error putting file... too less args".blue());
            continue
        }

        if input.starts_with("put") {
            let input_vec: Vec<_>  = input.split(" ").collect();
            if input_vec.len() == 2 {
                s3cli.put_file_encrypted(s3cli.bucket.clone(), input_vec[1].to_string(), input_vec[1].to_string()).await;
                continue
            }
            if input_vec.len() > 2 {
                s3cli.put_file_encrypted(input_vec[1].to_string(), input_vec[2].to_string(), input_vec[2].to_string()).await;
                continue
            }
            println!("{}", "error putting file... too less args".blue());
            continue
        }

        if input.starts_with("download") {
            let input_vec: Vec<_>  = input.split(" ").collect();
            if input_vec.len() > 2 {
                s3cli.get(input_vec[1].to_string(), input_vec[2].to_string(), input_vec[2].to_string()).await;
                continue
            }
            println!("{}", "error getting file... too less args".blue());
            continue
        }

        if input.starts_with("get") {
            let input_vec: Vec<_>  = input.split(" ").collect();
            if input_vec.len() == 2 {
                s3cli.get_file_encrypted(s3cli.bucket.clone(), input_vec[1].to_string(), input_vec[1].to_string()).await;
                continue
            }
            if input_vec.len() > 2 {
                s3cli.get_file_encrypted(input_vec[1].to_string(), input_vec[2].to_string(), input_vec[2].to_string()).await;
                continue
            }
            println!("{}", "error getting file... too less args".blue());
            continue
        }

        if input.starts_with("rm") || input.starts_with("del") {
            let input_vec: Vec<_>  = input.split(" ").collect();
            if input_vec.len() == 2 {
                if s3cli.bucket == "".to_string() || s3cli.bucket == "/" {
                    s3cli.rm(input_vec[1].to_string()).await;
                } else {
                    s3cli.rm_obj(s3cli.bucket.clone(), input_vec[1].to_string()).await;
                }

            } else if input_vec.len() == 3 {
                s3cli.rm_obj(input_vec[1].to_string(), input_vec[2].to_string()).await;
            } else  {
                println!("{}", "specify bucket name to remove bucket or bucket name and object name to remove object".yellow());
            }
            continue
        }

        if input.starts_with("config") {
            let input_vec: Vec<_>  = input.split(" ").collect();
            if input_vec.len() == 2 {
                if input_vec[1] == "cat" || input_vec[1] == "print" {
                    conf.print();
                }
                if input_vec[1] == "folder" {
                    println!("config folder: {}", conf.get_config_folder().green());
                }
                if input_vec[1] == "list" || input_vec[1] == "ls" {
                    conf.list();
                }
                if input_vec[1] == "create" || input_vec[1] == "add" || input_vec[1] == "new" {
                    S3Config::create();
                }
                if input_vec[1] == "delete" || input_vec[1] == "rm" || input_vec[1] == "del" {
                    let config_name = ask("Enter a name of config or filename to remove: ");
                    conf.clone().delete(config_name);
                }
                if input_vec[1] == "use" || input_vec[1] == "load" || input_vec[1] == "set" {
                    let config_name = ask("Enter a name of config or filename to use: ");
                    conf = S3Config::load(config_name);
                    s3cli.config = conf.clone();
                }
            } else if input_vec.len() == 3 {
                if input_vec[1] == "use" || input_vec[1] == "load" || input_vec[1] == "set" {
                    let config_name = input_vec[2].to_string();
                    conf = S3Config::load(config_name);
                    s3cli.config = conf.clone();
                }
                if input_vec[1] == "delete" || input_vec[1] == "rm" || input_vec[1] == "del" {
                    let config_name = input_vec[2].to_string();
                    conf.clone().delete(config_name);
                }
            } else {
                conf.print();
            }
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