use serde::{Serialize, Deserialize, ser::{Error, SerializeStruct}, Serializer};
use tracing_subscriber::registry::Data;

#[derive(sqlx::FromRow)]
pub struct Manga {
    pub id: i64,
    pub name: String,
    pub path: String,
}

//password & universal_path
pub struct Setting {
    pub id: i64,
    pub key: String,
    pub value: String,
}

#[derive(sqlx::FromRow)]
pub struct MangaFiles {
    pub id: i64,
    pub manga_id: i64,
    pub filename: String,
}

impl Serialize for MangaFiles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where  
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MangaFiles", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("manga_id", &self.manga_id)?;
        state.serialize_field("filename", &self.filename)?;
        state.end()
    }
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub db_settings: DatabaseSettings,
    pub server_settings: ServerSettings,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub url: String
}

#[derive(Deserialize, Debug )]
pub struct  ServerSettings {
    pub password: String,
    pub universal_path: String,
}