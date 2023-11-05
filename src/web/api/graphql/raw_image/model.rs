use async_graphql::SimpleObject;
use serde::Deserialize;

#[derive(Deserialize, SimpleObject)]
pub struct LustSizing {
    sizing_id: i32,
}

#[derive(Deserialize, SimpleObject)]
pub struct LustRes {
    pub image_id: String,
    processing_time: f64,
    io_time: f64,
    checksum: i64,
    images: Vec<LustSizing>,
    bucket_id: i64,
}

impl From<crate::services::lust::LustResponse> for LustRes {
    fn from(lr: crate::services::lust::LustResponse) -> Self {
        LustRes {
            image_id: lr.image_id,
            processing_time: lr.processing_time,
            io_time: lr.io_time,
            checksum: lr.checksum,
            images: lr
                .images
                .into_iter()
                .map(|s| LustSizing {
                    sizing_id: s.sizing_id,
                })
                .collect(),
            bucket_id: lr.bucket_id,
        }
    }
}
