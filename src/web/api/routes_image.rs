use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap};
use axum::response::IntoResponse;
use serde::Deserialize;
use tracing::error;

use crate::services::lust::Lust;

use crate::web::error::Error;

use super::ApiState;

#[derive(Deserialize)]
pub struct Image {
    size: Option<String>,
}

pub async fn get_image(
    State(context): State<ApiState>,
    Path(image_id): Path<String>,
    Query(payload): Query<Image>,
) -> Result<impl IntoResponse, Error> {
    let client = context.reqwest_client;
    let mut params = vec![("format".to_string(), "jpeg".to_string())];

    match payload.size {
        Some(size) => params.push(("size".to_string(), size)),
        None => (),
    }

    let response = Lust::get_file(&client, image_id, Some(params)).await;

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
