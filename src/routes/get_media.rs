use std::path::PathBuf;

#[get("/get-media")]
async fn get_media(manga_id: &str) {
    //TODO: Check if Client is Authenticated
    //TODO: Implement file server to serve manga file

    base:

}

async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok();
}