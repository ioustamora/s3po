use std::fs;
use std::process::exit;
use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use crate::console::{ask, y_or_n};
use crate::crypto::{gen_new_keys};
use chrono::offset::Utc;
use chrono::DateTime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct S3Config {
    pub(crate) base_url: String,
    pub(crate) access_key: String,
    pub(crate) secret_key: String,
    pub(crate) sk_bs58: String,
    pub(crate) pk_bs58: String,
}

impl S3Config {
    fn create() -> S3Config {
        println!("{}", "    Create new config ... ".red());
        let mut cfg: S3Config = S3Config::default();
        let mut config_name = ask("Please enter config name: ");
        cfg.base_url = ask("Please enter the s3 base url: ");
        cfg.access_key = ask("Please enter the s3 access key: ");
        cfg.secret_key = ask("Please enter the s3 secret key: ");
        let cfg = gen_new_keys(cfg);
        if config_name.trim() == ""  {
            config_name = "default".parse().unwrap();
        }
        confy::store("s3po", config_name, cfg.clone()).expect("error writing config ...");
        cfg
    }
    fn delete(config_name: String) {
        if !config_name.ends_with(".toml") {
            let config_name = config_name.clone() + ".toml";
        }
        let config_folder = Self::get_config_folder();
        let config_path = config_folder + "/" + &*config_name;
    }
    fn recreate_or_fix() -> S3Config {
        let config_path = confy::get_configuration_file_path("s3po", None).expect("can't get config path ...");
        println!("{}: {}", "can't load config from".blue(), config_path.to_str().unwrap());
        let recreate: bool = y_or_n("You want to rewrite new config file? (or fix it manually)");
        if recreate {
            return Self::create()
        } else {
            println!("{}: {}", "fix this config file manually".red(), config_path.to_str().unwrap());
            println!("{}", "buy...".yellow());
            exit(0);
        }
    }
    fn check(&self) -> bool {
        self.base_url.trim() == "" || self.access_key.trim() == "" || self.secret_key.trim() == "" || self.sk_bs58.trim() == "" || self.pk_bs58.trim() == ""
    }
    pub(crate) fn init() -> S3Config {
        let cfg = confy::load("s3po", None).unwrap_or_else(|error|
            Self::recreate_or_fix()
        );
        if  cfg.check() {
            return Self::recreate_or_fix();
        }
        let config_path = confy::get_configuration_file_path("s3po", None).expect("can't get config path ...");
        println!("{}: {}", "used config from".blue(), config_path.to_str().unwrap());
        println!();
        cfg
    }

    pub(crate) fn get_loaded_config_path() -> String {
        return String::from(confy::get_configuration_file_path("s3po", None).expect("can't get config path ...").to_str().unwrap())
    }

    pub(crate) fn get_config_folder() -> String {
        let path = String::from(confy::get_configuration_file_path("s3po", None).expect("can't get config path ...").to_str().unwrap());
        let mut input_vec: Vec<_>  = path.split("/").collect();
        input_vec.pop().unwrap();
        input_vec.join("/")
    }

    pub(crate) fn list() {
        match fs::read_dir(Self::get_config_folder().as_str()) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name().into_string().unwrap();
                        let time_modified: DateTime<Utc> = entry.metadata().unwrap().modified().unwrap().into();
                        let file_modified = time_modified.format("%d/%m/%Y %T");
                        println!("{} {}", file_name, file_modified);
                    }
                }
            },
            Err(error) => println!("Error reading directory: {}", error),
        }
    }

    pub(crate) fn print(&self) {
        println!("{}: {}", "loaded config".yellow(), S3Config::get_loaded_config_path().blue());
        println!("s3 server url: {}", self.base_url);
        println!("s3 access key: {}", self.access_key);
        println!("s3 secret key: {}", self.secret_key);
        println!("data encryption public key: {}", self.pk_bs58);
        println!("data encryption secret key: {}", self.sk_bs58);
    }
}

impl ::std::default::Default for S3Config {
    fn default() -> Self { Self { base_url: "".into(), access_key: "".into(), secret_key: "".into(), sk_bs58: "".into(), pk_bs58: "".into() } }
}



