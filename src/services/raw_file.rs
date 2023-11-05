use tokio::fs;

use crate::config;

use super::error::{Error, Result};

pub async fn read_dir(dir: &str) -> Result<Vec<String>> {
    let mut paths = fs::read_dir(dir)
        .await
        .map_err(|e| Error::FileSystem(format!("Error reading web folder: {}", e)))?;

    let mut dir_paths: Vec<String> = vec![];

    while let Some(path) = paths
        .next_entry()
        .await
        .map_err(|e| Error::FileSystem(format!("Error reading path in web folder: {}", e)))?
    {
        let path = path.file_name().into_string().map_err(|e| {
            let error_string = e.into_string().map_err(|_| {
                Error::FileSystem("Can't parse file system error to string".to_string())
            });
            match error_string {
                Ok(s) => Error::FileSystem(format!(
                    "Error converting path in web folder to string: {}",
                    s
                )),
                Err(e) => e,
            }
        })?;
        dir_paths.push(path);
    }

    Ok(dir_paths)
}

pub async fn read_root() -> Result<Vec<String>> {
    read_dir(&config().WEB_FOLDER).await
}

pub async fn read_album(dir: &str) -> Result<Vec<String>> {
    read_dir(&format!("{}/{}", &config().WEB_FOLDER, dir)).await
}

pub async fn read_file(file: &str) -> Result<Vec<u8>> {
    fs::read(&format!("{}/{}", &config().WEB_FOLDER, &file))
        .await
        .map_err(|e| Error::FileSystem(format!("Error {} reading file: {}", e, file)))
}
