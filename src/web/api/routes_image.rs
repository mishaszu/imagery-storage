use axum::extract::{BodyStream, Multipart, Path, Query, State};
use axum::headers::ContentLength;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{Json, TypedHeader};
use regex::Regex;
use serde::Deserialize;

use crate::config;
use crate::ctx::Ctx;
use crate::graphql::ImageKind;
use crate::model::account::AccountBmc;
use crate::model::image::{Image as ModelImage, ImageBmc, ImageForCreate};
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
    let mut params = vec![];

    match payload.size {
        Some(size) => params.push(("size".to_string(), size)),
        None => (),
    }

    let (response, headers) = Lust::get_file(
        &client,
        &config().LUST_IMAGE_BUCKET,
        &image_id,
        Some(params),
    )
    .await?;

    let mut file = response.into_response();
    let new_headers: &mut HeaderMap = file.headers_mut();
    headers.into_iter().for_each(|(k, v)| {
        if let Some(header_name) = k {
            new_headers.insert(header_name, v);
        }
    });

    Ok(file)
}

pub async fn post_image(
    TypedHeader(content_length): TypedHeader<ContentLength>,
    State(context): State<ApiState>,
    ctx: Ctx,
    file: BodyStream,
) -> Result<Json<ModelImage>, Error> {
    let mm = context.mm;
    let client = context.reqwest_client;
    let user = ctx.user_id;
    let user_account = ctx.account_id;

    let account = AccountBmc::get(&mm, &user_account)?;

    match account.kind.as_str() {
        "creator" => (),
        _ => {
            return Err(Error::BadRequest(
                "You must be a creator to upload images".to_string(),
            ))
        }
    }

    let size = content_length.0;
    let response = Lust::post_stream(&client, &config().LUST_IMAGE_BUCKET, size, file).await?;

    let image_id = response.image_id.parse()?;

    let image_for_create = ImageForCreate {
        id: uuid::Uuid::new_v4(),
        user_id: user.into(),
        name: None,
        kind: ImageKind::Image.to_string(),
        path: image_id,
    };

    let image = ImageBmc::create(&mm, image_for_create)?;

    Ok(image.into())
}

// TODO: Requires correct return type and backup transaction fallback for failed uploads
pub async fn post_images(
    State(context): State<ApiState>,
    ctx: Ctx,
    mut stream: Multipart,
) -> Result<String, Error> {
    let mm = context.mm;
    let client = context.reqwest_client;
    let user = ctx.user_id;

    let file_regex = Regex::new(r"file_(\d+)").unwrap();

    let mut images: Vec<(String, String)> = Vec::new();

    while let Some(field) = stream
        .next_field()
        .await
        .map_err(|e| Error::BadRequest(e.to_string()))?
    {
        match field.name() {
            Some(name) => match file_regex.find(name) {
                Some(_) => {
                    // TODO: would be better to pass stream to not buffer all files on server
                    // requires some work to retrive image size
                    let name = name.to_string();
                    let bytes = field
                        .bytes()
                        .await
                        .map_err(|e| Error::BadRequest(e.to_string()))?;

                    let response =
                        Lust::post_file(&client, &config().LUST_IMAGE_BUCKET, bytes.to_vec())
                            .await?;

                    let image_id = match response.image_id.parse() {
                        Ok(image_id) => image_id,
                        Err(_) => {
                            break;
                        }
                    };

                    let image = ImageBmc::create(
                        &mm,
                        crate::model::image::ImageForCreate {
                            id: uuid::Uuid::new_v4(),
                            user_id: user.into(),
                            name: None,
                            kind: ImageKind::Image.to_string(),
                            path: image_id,
                        },
                    );

                    match image {
                        Ok(image) => images.push((name, image.path.to_string())),
                        Err(_) => {
                            break;
                        }
                    }
                }
                None => (),
            },
            None => (),
        }
    }

    Ok("".to_string())
}
