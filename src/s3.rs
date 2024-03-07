use minio::s3::args::ListBucketsArgs;
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use crate::config::S3Config;

pub(crate) async fn test(cfg: S3Config) {
    let base_url: BaseUrl = cfg.base_url.parse::<BaseUrl>().expect("error parsing base url...");

    let static_provider = StaticProvider::new(
        &*cfg.access_key,
        &*cfg.secret_key,
        None,
    );

    let client = Client::new(
        base_url.clone(),
        Some(Box::new(static_provider)),
        None,
        None,
    )
        .unwrap();

    let buckets = client.list_buckets(&ListBucketsArgs::new()).await.unwrap();
    println!("{:?}", buckets);
}