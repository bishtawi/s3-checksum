use anyhow::Result;
use aws_sdk_s3::Endpoint;
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
