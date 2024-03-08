use std::process::exit;
use colored::Colorize;
use minio::s3::types::S3;
use serde_derive::{Deserialize, Serialize};
use crate::console::{ask, y_or_n};
use crate::crypto::new_keys;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct S3Config {
    pub(crate) base_url: String,
    pub(crate) access_key: String,
    pub(crate) secret_key: String,
    pub(crate) sk_bs58: String,
    pub(crate) pk_bs58: String,
}

impl S3Config {
    fn create() -> S3Config {
        let mut cfg: S3Config = S3Config::default();
        cfg.base_url = ask("Please enter the s3 base url: ");
        cfg.access_key = ask("Please enter the s3 access key: ");
        cfg.secret_key = ask("Please enter the s3 secret key: ");
        let (sk_bs58, pk_bs58) = new_keys();
        println!("{}","New cryptographic keys generated and will be saved in config".red());
        println!("{}: {}", "secret key".blue(), sk_bs58);
        println!("{}: {}", "public key".blue(), pk_bs58);
        cfg.sk_bs58 = sk_bs58;
        cfg.pk_bs58 = pk_bs58;

        confy::store("s3po", None, cfg.clone()).expect("error writing config ...");
        cfg
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
        cfg
    }
}

impl ::std::default::Default for S3Config {
    fn default() -> Self { Self { base_url: "".into(), access_key: "".into(), secret_key: "".into(), sk_bs58: "".into(), pk_bs58: "".into() } }
}



