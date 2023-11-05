use bytes::Bytes;
use reqwest::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Client, Url,
};
use serde::Deserialize;
use tracing::debug;

use crate::config;

use super::error::{Error, Result};
use super::req_client;

#[derive(Deserialize)]
pub struct LustSizing {
    pub sizing_id: i32,
}

#[derive(Deserialize)]
pub struct LustResponse {
    pub image_id: String,
    pub processing_time: f64,
    pub io_time: f64,
    pub checksum: i64,
    pub images: Vec<LustSizing>,
    pub bucket_id: i64,
}

pub struct Lust;

impl Lust {
    pub fn build_post_url(params: Option<Vec<(String, String)>>) -> Result<Url> {
        let url = &config().LUST_URL;
        req_client::build_url(url, params)
    }

    pub fn build_get_url(params: Option<Vec<(String, String)>>, image: String) -> Result<Url> {
        let url = format!("{}/{}", &config().LUST_URL, image);
        req_client::build_url(&url, params)
    }

    pub async fn post_file(client: &Client, file: Vec<u8>) -> Result<LustResponse> {
        let url = Self::build_post_url(Some(vec![("format".to_string(), "jpeg".to_string())]))?;
        debug!("{:<12} - LUST creating file", "REDIRECT");
        let res = client
            .post(url)
            .header(CONTENT_TYPE, "application/octet-stream")
            .header(CONTENT_LENGTH, file.len().to_string())
            .body(file)
            .send()
            .await
            .map_err(|e| e.into());

        match res {
            Ok(res) => match res.status().is_success() {
                true => res.json::<LustResponse>().await.map_err(|e| e.into()),
                false => Err(Error::LustResponse(res.status().to_string())),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn get_file(
        client: &Client,
        image_id: String,
        params: Option<Vec<(String, String)>>,
    ) -> Result<Bytes> {
        let url = Self::build_get_url(params, image_id)?;
        debug!("{:<12} - LUST getting file - {}", "REDIRECT", &url);
        let res = client.get(url).send().await.map_err(|e| e.into());

        match res {
            Ok(res) => {
                if res.status().is_success() {
                    let file: Bytes = res.bytes().await.unwrap();
                    Ok(file)
                } else {
                    let status = res.status().as_u16();
                    Err(Error::LustBadStatus(status))
                }
            }
            Err(e) => Err(Error::ReqwestFailed(e)),
        }
    }

    pub async fn delete_file(client: &Client, image_id: String) -> Result<()> {
        let url = Self::build_get_url(None, image_id)?;
        debug!("{:<12} - LUST deleting file", "REDIRECT");
        let res = client.delete(url).send().await.map_err(|e| e.into());

        match res {
            Ok(res) => match res.status().is_success() {
                true => {
                    debug!("{:<12} - LUST file deleted", "REDIRECT");
                    Ok(())
                }
                false => Err(Error::LustResponse(res.status().to_string())),
            },
            Err(e) => Err(e),
        }
    }
}
