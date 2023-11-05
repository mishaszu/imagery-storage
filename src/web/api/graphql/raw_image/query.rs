use std::fs::{self, File};

use async_graphql::{Context, Error, Object, Result};
use tracing::debug;

use crate::config;

#[derive(Default)]
pub struct RawImageQuery;

#[Object]
impl RawImageQuery {
    async fn raw_album(&self, dir: String) -> Result<Vec<String>> {
        let paths = fs::read_dir(format!("{}/{}", &config().WEB_FOLDER, dir))
            .map_err(|e| Error::new(format!("Error reading web folder: {}", e)))?;

        let mut dir_paths: Vec<String> = vec![];
        for path in paths {
            let path = path
                .map_err(|e| Error::new(format!("Error reading web folder: {}", e)))?
                .file_name()
                .into_string()
                .unwrap();
            dir_paths.push(path);
        }
        dir_paths.sort();

        Ok(dir_paths)
    }

    async fn raw_albums(&self, _ctx: &Context<'_>) -> Result<Vec<String>> {
        let paths = fs::read_dir(&config().WEB_FOLDER)
            .map_err(|e| Error::new(format!("Error reading web folder: {}", e)))?;

        let mut dir_paths: Vec<String> = vec![];
        for path in paths {
            let path = path
                .map_err(|e| Error::new(format!("Error reading web folder: {}", e)))?
                .file_name()
                .into_string()
                .unwrap();
            dir_paths.push(path);
        }

        Ok(dir_paths)
    }

    async fn raw_image_size(&self, dir: String, file: String) -> Result<i32> {
        debug!("Reading raw image: {}/{}", dir, file);
        let path = format!("{}/{}/{}", &config().WEB_FOLDER, dir, file);

        let my_file =
            File::open(path).map_err(|e| Error::new(format!("Error reading file: {}", e)))?;
        let size = my_file.metadata().unwrap().len() as i32;

        Ok(size)
    }
}
