pub mod models;

use std::result::Result;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};

pub enum DbErrors {
    SqlxError(sqlx::Error),
}

impl From<sqlx::Error> for DbErrors{
    fn from(e: sqlx::Error) -> Self {
        Self::SqlxError(e)
    }
}

pub async fn create_db(db_url: &str) -> Result<(), DbErrors>{

    let pool = SqlitePool::connect(&db_url).await?;

    if let Err(_) = Sqlite::database_exists(&db_url).await {
        match create_schema(&db_url).await {
            Ok(_) => Ok(()),
            Err(e) => Err(DbErrors::SqlxError(e)),
        }
    }
    else{
        return Err(DbErrors::SqlxError(sqlx::Error::Protocol("No Database Exists".to_owned())));
    }
}

async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(&db_url).await?;
    let qry =
    "PRAGMA foreign_keys = ON;
    CREATE TABLE IF NOT EXISTS manga
        (
            id          INTEGER PRIMARY KEY NOT NULL,
            name        TEXT                NOT NULL,
            path        TEXT                NOT NULL,
        );
    CREATE TABLE IF NOT EXISTS setting
        (
            id          INTEGER PRIMARY KEY NOT NULL,
            key         TEXT                NOT NULL,
            value       TEXT                NOT NULL,
        );
    CREATE TABLE IF NOT EXISTS manga_files
        (
            id          INTEGER PRIMARY KEY NOT NULL,
            manga_id    INTEGER             NOT NULL,
            filename    TEXT                NOT NULL,
        );
    ";

    let result = sqlx::query(&qry).execute(&pool).await;   
    pool.close().await; 
    return result;
}