use std::fs;
use colored::Colorize;
use minio::s3::args::{BucketExistsArgs, DownloadObjectArgs, GetObjectArgs, ListBucketsArgs, ListObjectsV2Args, MakeBucketArgs, PutObjectApiArgs, PutObjectArgs, RemoveBucketArgs, UploadObjectArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::response::PutObjectApiResponse;
use minio::s3::types::Bucket;
use crate::config::S3Config;
use crate::crypto::{decrypt_bytes, encrypt_bytes};

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

    pub(crate) async fn set_bucket (&mut self, bucket: String) {
        if bucket == String::from("..") || bucket == String::from("/") {
            self.bucket = String::from("");
            return;
        }

        self.bucket = bucket;
    }

    pub(crate) async fn get_bucket (&mut self, bucket: String) -> String {
        self.bucket.clone()
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

    pub(crate) async fn put_file_encrypted(&self, bucket_name: String, remote_file_name: String, local_file_path: String) {
        let file_bytes = fs::read(local_file_path.to_string())
            .expect("can't open file for encryption and upload");
        self.put_bytes_encrypted(bucket_name, remote_file_name, file_bytes).await;
    }

    pub(crate) async fn put_bytes_encrypted(&self, bucket_name: String, remote_file_name: String, file_bytes: Vec<u8>) {
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
                    let remote_file_name = remote_file_name + ".x";
                    let conf = self.config.clone();
                    let encrypted_bytes = encrypt_bytes(conf, file_bytes);
                    let resp = client.put_object_api(&PutObjectApiArgs::new(&*bucket_name, &*remote_file_name, &*encrypted_bytes).unwrap()).await;
                    match resp {
                        Ok(resp) => {
                            println!("file: {} successfully saved to bucket: {}", resp.object_name, resp.bucket_name);
                        }
                        Err(err) => {
                            println!("error putting bytes to file {}", remote_file_name);
                        }
                    }
                } else {
                    println!("bucket {} does not exists", bucket_name);
                }
            }
            Err(err) => {
                println!("error while checking existence of bucket: {}", bucket_name);
                return;
            }
        }
    }

    pub(crate) async fn get_file_encrypted(&self, bucket_name: String, remote_file_name: String, local_file_path: String) {
        let file_bytes = self.get_bytes_encrypted(bucket_name, remote_file_name.clone()).await;
        let empty_vec: Vec<u8> = vec![];
        if file_bytes != empty_vec {
            fs::write(local_file_path.strip_suffix(".x").unwrap(), file_bytes).expect("error writing decrypted file");
            println!("file {} successfully downloaded and decrypted to {}", remote_file_name, local_file_path);
        }
    }

    pub(crate) async fn get_bytes_encrypted(&self, bucket_name: String, remote_file_name: String) -> Vec<u8> {
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
        return match exists {
            Ok(exist) => {
                if exist {
                    let resp = client.get_object(&GetObjectArgs::new(&*bucket_name, &*remote_file_name).unwrap()).await;
                    match resp {
                        Ok(resp) => {
                            let conf = self.config.clone();
                            let decrypted_bytes = decrypt_bytes(conf, resp.bytes().await.unwrap().to_vec());
                            decrypted_bytes
                        }
                        Err(err) => {
                            println!("error getting bytes from file {} in bucket {}", remote_file_name, bucket_name);
                            vec![]
                        }
                    }
                } else {
                    println!("bucket {} does not exists", bucket_name);
                    vec![]
                }
            }
            Err(err) => {
                println!("error while checking existence of bucket: {}", bucket_name);
                vec![]
            }
        }
    }
}

impl ::std::default::Default for S3Client {
    fn default() -> Self { Self { config: S3Config::default(), bucket: "".to_string() } }
}
