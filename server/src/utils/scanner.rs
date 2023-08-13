use sqlx::{
    SqlitePool,
    FromRow,
};
use std::fs;
use crate::utils::env_load::get_universal_path;
use crate::routes::scan::ScanErrors;

//#derive[(Debug)]
pub enum ScanUtilErrors {
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for ScanUtilErrors {
    fn from(e: sqlx::Error) -> Self {
        ScanUtilErrors::DatabaseError(e)
    }
}

impl From<ScanUtilErrors> for ScanErrors {
    fn from(e: ScanUtilErrors) -> Self {
        match e {
            ScanUtilErrors::DatabaseError(sqlx_err) =>
                Self::SqlxError(sqlx_err),
        }
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
    sqlx::query(
        "INSERT INTO manga_files (manga_id, filename) VALUES ($1, $2)",
    )
    .bind(manga_id)
    .bind(filename)
    .execute(pool).await?;

    Ok(())
}

pub async fn scan_dir(pool: &SqlitePool) -> Result<(), ScanUtilErrors> {
    let path = get_universal_path();

    // Read the directory
    //let entries = fs::read_dir(path)?;
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(io_err) => return Err(ScanUtilErrors::DatabaseError(sqlx::Error::Io(io_err))),
    };

    let id = get_next_manga_id(pool).await?;

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(io_err) => return Err(ScanUtilErrors::DatabaseError(sqlx::Error::Io(io_err))),
        };
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
