use colored::Colorize;
use minio::s3::args::{BucketExistsArgs, DownloadObjectArgs, ListBucketsArgs, ListObjectsV2Args, MakeBucketArgs, PutObjectArgs, RemoveBucketArgs, UploadObjectArgs};
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
    async fn list_buckets(&self) {
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
                    println!("  {} {}", bucket.name, bucket.creation_date)
                }
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }

    async fn list_objects(&self, bucket_name: String) {
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

        let objects = client.list_objects_v2(&ListObjectsV2Args::new(&*bucket_name.clone()).unwrap()).await;
        match objects {
            Ok(objects) => {
                for object in objects.contents {
                    println!("  {} {} {}", object.name, object.size.unwrap(), object.last_modified.unwrap())
                }
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }
    pub(crate) async fn ls(&self, bucket_name: String) {
        if bucket_name.trim() == "" || bucket_name.trim() == "/" {
            self.list_buckets().await;
            return
        }

        self.list_objects(bucket_name.clone()).await;
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
                return;
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

    pub(crate) async fn rm(&self, bucket_name: String) {
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
                    let resp = client.remove_bucket(&RemoveBucketArgs::new(&*bucket_name.clone()).unwrap()).await;
                    match resp {
                        Ok(resp) => {
                            println!("bucket {} successfully deleted", resp.bucket_name);
                        }
                        Err(err) => {
                            println!("cant delete/remove bucket with name: {}", bucket_name);
                        }
                    }
                    return;
                } else {
                    println!("bucket with name {} not exists", bucket_name);
                    return;
                }
            }
            Err(err) => {
                println!("cant check existence of bucket with name: {}", bucket_name);
                return;
            }
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
            .bucket_exists(&BucketExistsArgs::new(&*bucket_name.clone()).unwrap())
            .await;
        match exists {
            Ok(exist) => {
                if exist {
                    let resp = client.download_object(&DownloadObjectArgs::new(&*bucket_name.clone(), &*remote_file_name.clone(), &*local_file_path.clone()).unwrap()).await;
                    match resp {
                        Ok(resp) => {
                            println!("file: {} downloaded from bucket: {} successfully ", resp.object_name, resp.bucket_name);
                        }
                        Err(err) => {
                            println!("cant load file: {} from bucket: {}", remote_file_name, bucket_name);
                        }
                    }
                    return;
                } else {
                    println!("bucket with name {} not exists", bucket_name);
                }
            }
            Err(err) => {
                println!("cant check existence of bucket with name: {}", bucket_name);
                return;
            }
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
            .bucket_exists(&BucketExistsArgs::new(&*bucket_name.clone()).unwrap())
            .await;
        match exists {
            Ok(exist) => {
                if exist {
                    let resp = client.upload_object(&UploadObjectArgs::new(&*bucket_name.clone(), &*remote_file_name.clone(), &*local_file_path.clone()).unwrap()).await;
                    match resp {
                        Ok(resp) => {
                            println!("file: {} uploaded to bucket: {} successfully ", resp.object_name, resp.bucket_name);
                        }
                        Err(err) => {
                            println!("cant put file: {} to bucket: {}", remote_file_name, bucket_name);
                        }
                    }
                }
            }
            Err(err) => {
                println!("cant check existence of bucket with name: {}", bucket_name);
                return;
            }
        }

    }
}

impl ::std::default::Default for S3Client {
    fn default() -> Self { Self { config: S3Config::default(), bucket: "".to_string() } }
}
