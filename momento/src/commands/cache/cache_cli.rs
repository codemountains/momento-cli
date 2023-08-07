use log::debug;
use std::process::exit;
use std::time::Duration;

use crate::{
    error::CliError,
    utils::{
        client::{get_momento_client, interact_with_momento},
        console::console_data,
    },
};

pub async fn create_cache(
    cache_name: String,
    auth_token: String,
    endpoint: Option<String>,
) -> Result<(), CliError> {
    let mut client = get_momento_client(auth_token, endpoint).await?;

    interact_with_momento("creating cache...", client.create_cache(&cache_name)).await
}

pub async fn delete_cache(
    cache_name: String,
    auth_token: String,
    endpoint: Option<String>,
) -> Result<(), CliError> {
    let mut client = get_momento_client(auth_token, endpoint).await?;

    interact_with_momento("deleting cache...", client.delete_cache(&cache_name)).await
}

pub async fn list_caches(auth_token: String, endpoint: Option<String>) -> Result<(), CliError> {
    let mut client = get_momento_client(auth_token, endpoint).await?;

    let list_result = interact_with_momento("listing caches...", client.list_caches(None)).await?;

    list_result
        .caches
        .into_iter()
        .for_each(|cache| console_data!("{}", cache.cache_name));

    Ok(())
}

pub async fn flush_cache(
    cache_name: String,
    auth_token: String,
    endpoint: Option<String>,
) -> Result<(), CliError> {
    let mut client = get_momento_client(auth_token, endpoint).await?;
    client.flush_cache(&cache_name).await?;
    Ok(())
}

pub async fn set(
    cache_name: String,
    auth_token: String,
    key: String,
    value: String,
    ttl_seconds: u64,
    endpoint: Option<String>,
) -> Result<(), CliError> {
    debug!("setting key: {} into cache: {}", key, cache_name);
    let mut client = get_momento_client(auth_token, endpoint).await?;

    interact_with_momento(
        "setting...",
        client.set(
            &cache_name,
            key,
            value,
            Some(Duration::from_secs(ttl_seconds)),
        ),
    )
    .await
    .map(|_| ())
}

pub async fn get(
    cache_name: String,
    auth_token: String,
    key: String,
    endpoint: Option<String>,
) -> Result<(), CliError> {
    debug!("getting key: {} from cache: {}", key, cache_name);

    let mut client = get_momento_client(auth_token, endpoint).await?;

    let response = interact_with_momento("getting...", client.get(&cache_name, key)).await?;
    match response {
        momento::response::Get::Hit { value } => {
            let value: String = value.try_into()?;
            console_data!("{}", value);
        }
        momento::response::Get::Miss => {
            debug!("cache miss");
            exit(1)
        }
    };
    Ok(())
}

pub async fn delete_key(
    cache_name: String,
    auth_token: String,
    key: String,
    endpoint: Option<String>,
) -> Result<(), CliError> {
    debug!("deleting key: {} from cache: {}", key, cache_name);

    let mut client = get_momento_client(auth_token, endpoint).await?;

    interact_with_momento(
            "deleting...",
            client.delete(
                &cache_name,
                key
            ),
        )
        .await
        .map(|_| ())
}
