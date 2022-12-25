use std::path::PathBuf;
use glob::glob;
use rocket::State;
use rocket_contrib::database::DbConn;

fn scan_directory(conn: &DbConn, path: &PathBuf) -> Result<(), diesel::result::Error> {
    let mut found_mangas = Vec::new();

    // Scan the directory for CBZ, CBR, and PDF files
    let cbz_pattern = path.join("**/*.cbz");
    let cbr_pattern = path.join("**/*.cbr");
    let pdf_pattern = path.join("**/*.pdf");

    for entry in glob(&cbz_pattern.to_string_lossy())? {
        let entry = entry?;
        let path = entry.path();
        found_mangas.push(NewManga {
            path: path.to_string_lossy().to_string(),
            file_format: "cbz".to_string(),
        });
    }
    for entry in glob(&cbr_pattern.to_string_lossy())? {
        let entry = entry?;
        let path = entry.path();
        found_mangas.push(NewManga {
            path: path.to_string_lossy().to_string(),
            file_format: "cbr".to_string(),
        });
    }
    for entry in glob(&pdf_pattern.to_string_lossy())? {
        let entry = entry?;
        let path = entry.path();
        found_mangas.push(NewManga {
            path: path.to_string_lossy().to_string(),
            file_format: "pdf".to_string(),
        });
    }

    // Insert the found manga entries into the database
    use schema::mangas::dsl::*;
    for manga in found_mangas {
        diesel::insert_into(mangas)
            .values(&manga)
            .execute(conn)?;
    }

    Ok(())
}
