mod console;
mod config;
mod s3;

use minio::s3::args::{BucketExistsArgs, MakeBucketArgs, UploadObjectArgs, ListBucketsArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use crate::console::{print_help, print_intro, console_loop};
use crate::config::S3Config;


#[::tokio::main]
async fn main() {
    print_intro();
    console_loop().await;
}