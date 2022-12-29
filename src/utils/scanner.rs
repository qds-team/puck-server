use std::fs;
use std::path::Path;
use std::time::Duration;

use tokio::time::delay_for;

use crate::db::models::{Manga, MangaFile, NewManga, NewMangaFile};
use crate::schema::manga_files::dsl::*;

pub async fn scan_new_manga_files(conn: &SqliteConnection, manga_id: i32, path: &str) {
    let manga = Manga::get(manga_id, conn).unwrap();
    let manga_path = Path::new(&manga.path);

    let files = fs::read_dir(path).unwrap();

    for file in files {
        let file = file.unwrap();
        let file_path = file.path();
        let file_extension = file_path.extension().unwrap().to_str().unwrap();

        if file_extension == "cbz" || file_extension == "cbr" || file_extension == "pdf" {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let manga_file_path = manga_path.join(file_name);
            let manga_file_path = manga_file_path.to_str().unwrap();

            let new_manga_file = NewMangaFile {
                manga_id: manga_id,
                path: manga_file_path.to_string(),
            };

            let manga_file = NewMangaFile::insert(new_manga_file, conn);
        }
    }
}
