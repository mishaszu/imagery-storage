use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap};
use axum::response::IntoResponse;
use serde::Deserialize;
use tracing::error;

use super::GraphQlState;
use crate::services::lust::Lust;

use crate::services::raw_file::read_file;
use crate::web::error::Error;

#[derive(Deserialize)]
pub struct Image {
    image_id: String,
    size: Option<String>,
}

pub async fn get_image(
    State(context): State<GraphQlState>,
    Query(payload): Query<Image>,
) -> Result<impl IntoResponse, Error> {
    let client = context.reqwest_client;

    let mut params = vec![("format".to_string(), "jpeg".to_string())];

    match payload.size {
        Some(size) => params.push(("size".to_string(), size)),
        None => (),
    }

    let response = Lust::get_file(&client, payload.image_id, Some(params)).await;

    match response {
        Ok(res) => {
            let mut file = res.into_response();
            let headers: &mut HeaderMap = file.headers_mut();
            headers.insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("image/jpeg"),
            );
            Ok(file)
        }
        Err(_) => {
            error!("{:<12} - failed to redirect", "REDIRECT",);
            Err(Error::FailedToRedirect)
        }
    }
}

pub async fn get_raw_image(
    Path((album_id, image_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let image = format!("{}/{}", album_id, image_id);
    let file = read_file(&image).await.map_err(|e| {
        error!("Failed to read file: {:?}", e);
        Error::FailedToReadFile
    });

    match file {
        Ok(file) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("image/jpeg"),
            );

            Ok((headers, file))
        }
        Err(e) => Err(e),
    }
}
