use std::fs;
use std::path::Path;

use diesel::prelude::*;

use crate::models::{Manga, NewManga};
use crate::schema::mangas::dsl::*;

pub fn scan_directory(conn: &SqliteConnection, universal_path: &str) -> Result<(), diesel::result::Error> {
    let directory = Path::new(universal_path);
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        let existing_manga = mangas
            .filter(name.eq(file_name))
            .first::<Manga>(conn);

        if existing_manga.is_err() {
            let new_manga = NewManga::new(file_name, path.to_str().unwrap(), universal_path);
            diesel::insert_into(mangas)
                .values(&new_manga)
                .execute(conn)?;
        }
    }

    Ok(())
}
