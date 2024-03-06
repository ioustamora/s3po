use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use crate::console::ask;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct S3Config {
    pub(crate) base_url: String,
    pub(crate) access_key: String,
    pub(crate) secret_key: String,
}

impl S3Config {
    pub(crate) fn init_config() -> S3Config {
        let mut cfg: S3Config = confy::load("s3po", None).expect("cant load config ...");
        if cfg.base_url == "" {
            println!("{}", "base url not set...".blue());
            cfg.base_url = ask("Please enter the s3 base url: ");
            confy::store("s3po", None, cfg.clone()).expect("error writing config ...");
        }
        if cfg.access_key == "" {
            println!("{}", "access key not set...".blue());
            cfg.access_key = ask("Please enter the s3 access key: ");
            confy::store("s3po", None, cfg.clone()).expect("error writing config ...");
        }
        if cfg.secret_key == "" {
            println!("{}", "secret key not set...".blue());
            cfg.secret_key = ask("Please enter the s3 secret key: ");
            confy::store("s3po", None, cfg.clone()).expect("error writing config ...");
        }
        let config_path = confy::get_configuration_file_path("s3po", None).expect("can't get config path ...");
        println!("loaded config from: {:?}", config_path);

        return cfg;
    }
}

impl ::std::default::Default for S3Config {
    fn default() -> Self { Self { base_url: "".into(), access_key: "".into(), secret_key: "".into() } }
}



