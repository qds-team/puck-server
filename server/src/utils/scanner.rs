use sqlx::{
    SqlitePool,
    FromRow,
};
use std::fs;
use crate::utils::env_load::get_universal_path;

//#derive[(Debug)]
enum ScanUtilErrors {
    DatabaeError(sqlx::Error),
}

impl From<sqlx::Error> for ScanUtilErrors {
    fn from(e: sqlx::Error) -> Self {
        ScanUtilErrors::DatabaseError(e)
    }
}
pub async fn get_next_manga_id(pool: &SqlitePool) -> Result<i32, ScanUtilErrors> {
    let highest_manga_id: Option<i32> = sqlx::query_scalar(
        "SELECT MAX(manga_id) FROM manga_files"
    )
    .fetch_one(pool)
    .await?;

    Ok(match highest_manga_id{
        Some(max_id) => max_id + 1,
        None => 1,
    })
}

pub async fn add_manga_db(
    pool: &SqlitePool,
    manga_id: i32,
    filename: &str
) -> Result<(), ScanUtilErrors> {
    sqlx::query_as(
        "INSERT INTO manga_files (manga_id, filename) VALUES ($1, $2)",
    )
    .bind(manga_id)
    .bind(filename)
    .fetch_one(&mut pool.acquire().await?)
    .await
    .map_err(|err| ScanUtilErrors::DatabaseError(err))?;

    Ok(())
}

pub async fn scan_dir(pool: &SqlitePool) -> Result<(), ScanUtilErrors> {
    let path = get_universal_path();

    // Read the directory
    let entries = fs::read_dir(path)?;

    let id = get_next_manga_id(pool).await?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();

        //Check if file_name exists in DB
        let exists_in_db: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM manga_files WHERE filename = $1)"
        )
        .bind(file_name)
        .fetch_one(pool)
        .await?;

        if !exists_in_db {
            add_manga_db(pool, id, file_name).await?;
            println!("Added {} to the database", file_name);
        }
        else {
            println!("{} already exists in the database.", file_name);
        }
    }

    Ok(())
}
