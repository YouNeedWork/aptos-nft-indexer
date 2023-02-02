use anyhow::Result;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{config::Builder, Client, Credentials, Region};
use std::path::Path;

use crate::config::IndexConfig;

pub async fn upload_object(
    client: &Client,
    bucket_name: &str,
    file_name: &str,
    key: &str,
) -> Result<()> {
    let body = ByteStream::from_path(Path::new(file_name)).await;

    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body.unwrap())
        .send()
        .await?;

    println!("Uploaded file: {}", file_name);
    Ok(())
}

pub async fn get_client(cfg: &IndexConfig, region: &str) -> Result<Client> {
    // build the aws cred
    let cred = Credentials::new(
        cfg.key.clone(),
        cfg.secret.clone(),
        None,
        None,
        "loaded-from-custom-env",
    );

    // build the aws client
    let region = Region::new(region.to_string());
    let conf_builder = Builder::new().region(region).credentials_provider(cred);

    let conf = conf_builder.build();

    // build aws client
    let client = Client::from_conf(conf);

    Ok(client)
}
