use minio::s3::args::ListBucketsArgs;
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use crate::config::S3Config;

pub(crate) struct S3Client {
    pub(crate) config: S3Config,
}

impl S3Client {
    pub(crate) async fn ls(&self) {
        let base_url: BaseUrl = self.config.base_url.parse::<BaseUrl>().expect("error parsing base url...");

        let static_provider = StaticProvider::new(
            &*self.config.access_key,
            &*self.config.secret_key,
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
}

impl ::std::default::Default for S3Client {
    fn default() -> Self { Self { config: S3Config::default() } }//{ base_url: "".into(), access_key: "".into(), secret_key: "".into(), sk_bs58: "".into(), pk_bs58: "".into() }
}
