use std::env;
use std::process::exit;
use colored::Colorize;
use crate::config::S3Config;
use crate::s3::S3Client;

pub(crate) async fn parse_args() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].trim() == "console" {
        return;
    }

    println!("command line mode");

    let mut conf: S3Config = S3Config::init();
    let mut s3cli = S3Client{ config: conf.clone(), bucket: "".to_string() };

    if args[1].trim() == "ls" {
        println!("ls");
        if args.len() == 3 {
            s3cli.ls(args[2].trim().to_string()).await;
        } else {
            s3cli.ls("".to_string()).await;
        }
    } else if args[1].trim() == "mkdir" {
        println!("mkdir");
    } else if args[1].trim() == "get" {
        println!("get");
        if args.len() > 3 {
            s3cli.get_file_encrypted(args[2].to_string(), args[3].to_string(), args[3].to_string()).await;
        } else {
            println!("{}", "error getting file... too less args".blue());
        }
    } else if args[1].trim() == "put" {
        println!("put");
        if args.len() > 3 {
            s3cli.put_file_encrypted(args[2].to_string(), args[3].to_string(), args[3].to_string()).await;
        } else {
            println!("{}", "error putting file... too less args".blue());
        }
    } else if args[1].trim() == "rm" {
        println!("rm");
    } else if args[1].trim() == "config" {
        println!("config");
    }

    exit(0);
}