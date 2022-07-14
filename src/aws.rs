use anyhow::Result;
use aws_sdk_s3::{model::Object, Endpoint};
use aws_types::SdkConfig;

pub async fn get_aws_config(aws_endpoint: &Option<String>) -> Result<SdkConfig> {
    let config = if let Some(endpoint) = aws_endpoint {
        aws_config::from_env()
            .endpoint_resolver(Endpoint::immutable(endpoint.parse()?))
            .load()
            .await
    } else {
        aws_config::load_from_env().await
    };

    Ok(config)
}

pub async fn crawl_bucket(
    s3_client: &aws_sdk_s3::Client,
    bucket: &String,
    path: &Option<String>,
) -> Result<Vec<Object>> {
    let mut s3_objects: Vec<Object> = Vec::new();
    let mut continuation_token: Option<String> = None;
    loop {
        let resp = s3_client
            .list_objects_v2()
            .bucket(bucket)
            .set_prefix(path.clone())
            .set_continuation_token(continuation_token)
            .send()
            .await?;

        if let Some(mut contents) = resp.contents {
            s3_objects.append(&mut contents);
        }

        if resp.continuation_token.is_none() {
            break;
        }

        continuation_token = resp.continuation_token;
    }

    Ok(s3_objects)
}
