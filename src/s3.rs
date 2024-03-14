use colored::Colorize;
use minio::s3::args::{BucketExistsArgs, ListBucketsArgs, MakeBucketArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::types::Bucket;
use crate::config::S3Config;

pub(crate) struct S3Client {
    pub(crate) config: S3Config,
    pub(crate) bucket: String,
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

        let buckets = client.list_buckets(&ListBucketsArgs::new()).await;
        match buckets {
            Ok(buckets) => {
                for bucket in buckets.buckets {
                    println!("{} {}", bucket.name, bucket.creation_date)
                }
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }

    pub(crate) async fn mkdir(&self, bucket_name: String) {
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
            .expect("error creating s3 client");

        let exists = client
            .bucket_exists(&BucketExistsArgs::new(&*bucket_name.clone()).unwrap())
            .await;
        match exists {
            Ok(exist) => {
                if exist {
                    println!("bucket with name {} already exists", bucket_name);
                    return;
                }
            }
            Err(err) => {
                println!("cant check existence of bucket with name: {}", bucket_name);
            }
        }

        let resp = client.make_bucket(&MakeBucketArgs::new(&*bucket_name.clone()).unwrap()).await;
        match resp {
            Ok(resp) => {
                println!("bucket {} successfully created", resp.bucket_name);
            }
            Err(err) => {
                println!("cant create bucket with name: {}", bucket_name);
            }
        }
    }
    pub(crate) async fn cd(&mut self, bucket_name: String) {
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

        let exists = client
            .bucket_exists(&BucketExistsArgs::new(&bucket_name).unwrap())
            .await
            .unwrap();

        if !exists && bucket_name.trim() != "" {
            self.bucket = bucket_name;
        }
    }
    pub(crate) async fn get(&self, bucket_name: String, remote_file_name: String, local_file_path: String) {
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

        let exists = client
            .bucket_exists(&BucketExistsArgs::new(&bucket_name).unwrap())
            .await
            .unwrap();

        if !exists && bucket_name.trim() != "" {

        }
    }
    pub(crate) async fn put(&self, bucket_name: String, remote_file_name: String, local_file_path: String) {
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

        let exists = client
            .bucket_exists(&BucketExistsArgs::new(&bucket_name).unwrap())
            .await
            .unwrap();
    }
}

impl ::std::default::Default for S3Client {
    fn default() -> Self { Self { config: S3Config::default(), bucket: "".to_string() } }//{ base_url: "".into(), access_key: "".into(), secret_key: "".into(), sk_bs58: "".into(), pk_bs58: "".into() }
}
