use std::ptr::hash;
use super::schema::{mangas, settings, users};
use serde::{Deserialize, Serialize};
use diesel::{self, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use bcrypt::{hash, verify};
use crate::db::schema::mangas::dsl::mangas;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Manga {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub files: Vec<MangaFiles>,
}


#[derive(Insertable)]
#[table_name = "manga_files"]
pub struct NewManga {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub files: Vec<MangaFiles>,
}

impl Manga {
    pub fn all(conn: &SqliteConnection) -> Vec<Manga> {
        use schema::manga::dsl::*;

        manga.load::<Manga>(conn).unwrap()
    }

    pub fn get(id: i32, conn: &SqliteConnection) -> Option<Manga> {
        use schema::manga::dsl::*;

        manga.find(id).first::<Manga>(conn).ok()
    }

    pub fn insert(manga: NewManga, conn: &SqliteConnection) -> Result<Manga, DieselError> {
        use schema::manga::dsl::*;

        diesel::insert_into(manga::table)
            .values(&manga)
            .get_result::<Manga>(conn)
    }

    pub fn update(id: i32, manga: NewManga, conn: &SqliteConnection) -> bool {
        use schema::manga::dsl::*;

        diesel::update(manga.find(id))
            .set(&manga)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(id: i32, conn: &SqliteConnection) -> bool {
        use schema::manga::dsl::*;

        diesel::delete(manga.find(id))
            .execute(conn)
            .is_ok()
    }
}

impl Insertable<manga::table> for Manga {
    fn values(&self) -> crate::rocket_contrib::databases::diesel::InsertValues<manga::table> {
        use crate::rocket_contrib::databases::diesel::dsl::{columns, values};

        values((
            manga::name.eq(self.name.clone()),
            manga::path.eq(self.path.clone()),
        ))
    }
}

impl AsChangeset for Manga {
    fn as_changeset(&self) -> Changeset {
        use crate::rocket_contrib::databases::diesel::dsl::{columns, values};

        values((
            manga::name.eq(self.name.clone()),
            manga::path.eq(self.path.clone()),
        ))
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Setting {
    pub id: i32,
    pub key: String,
    pub value: String,
}

#[derive(Insertable)]
#[table_name="settings"]
pub struct NewSetting {
    pub key: String,
    pub value: String,
}

impl Setting {
    pub fn all(conn: &SqliteConnection) -> Vec<Setting> {
        use crate::schema::settings::dsl::*;

        settings.load::<Setting>(conn).unwrap()
    }

    pub fn get(conn: &SqliteConnection, key: &str) -> Result<Setting, String> {
        use crate::schema::settings::dsl::*;

        settings.filter(key.eq(key)).first::<Setting>(conn).map_err(|e| e.to_string())
    }

    pub fn insert(conn: &SqliteConnection, setting: NewSetting) -> Result<Setting, String> {
        use crate::schema::settings::dsl::*;

        diesel::insert_into(settings).values(&setting).execute(conn).map_err(|e| e.to_string())?;

        settings.order(id.desc()).first::<Setting>(conn).map_err(|e| e.to_string())
    }

    pub fn update(conn: &SqliteConnection, key: &str, setting: NewSetting) -> Result<(), String> {
        use crate::schema::settings::dsl::*;

        diesel::update(settings.filter(key.eq(key)))
            .set((key.eq(&setting.key), value.eq(&setting.value)))
            .execute(conn)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub ip: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub ip: String,
    pub password_hash: String,
}

impl User {
    pub fn find(conn: &SqliteConnection, ip: &str) -> Result<User, String> {
        use crate::schema::users::dsl::*;

        users.filter(ip.eq(ip)).first::<User>(conn).map_err(|e| e.to_string())
    }

    pub fn register(conn: &SqliteConnection, ip: &str, password: &str) -> Result<User, String> {
        use crate::schema::users::dsl::*;

        let password_hash = hash(password, 6).map_err(|e| e.to_string())?;
        let new_user = NewUser {
            ip: ip.to_string(),
            password_hash,
        };

        diesel::insert_into(users).values(&new_user).execute(conn).map_err(|e| e.to_string())?;

        users.order(id.desc()).first::<User>(conn).map_err(|e| e.to_string())
    }

    pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, String> {
        verify(password, password_hash).map_err(|e| e.to_string())
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct MangaFiles {
    pub id: i32,
    pub manga_id: i32,
    pub file_name: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "manga_files"]
pub struct NewMangaFiles {
    pub manga_id: i32,
    pub file_name: String,
}

impl MangaFiles {
    pub fn all(connection: &SqliteConnection) -> QueryResult<Vec<MangaFiles>> {
        manga_files::table.load::<MangaFiles>(connection)
    }

    pub fn get(id: i32, connection: &SqliteConnection) -> QueryResult<MangaFiles> {
        manga_files::table.find(id).first(connection)
    }

    pub fn insert(manga_file: NewMangaFiles, connection: &SqliteConnection) -> QueryResult<MangaFiles> {
        diesel::insert_into(manga_files::table)
            .values(manga_file)
            .execute(connection)?;

        manga_files::table
            .order(manga_files::id.desc())
            .first(connection)
    }

    pub fn update(manga_file: &MangaFiles, connection: &SqliteConnection) -> QueryResult<MangaFiles> {
        diesel::update(manga_files::table.find(manga_file.id))
            .set(manga_file)
            .execute(connection)?;

        manga_files::table.find(manga_file.id).first(connection)
    }
}

impl MangaFiles {
    pub fn insert(manga_files: NewMangaFiles, conn: &SqliteConnection) -> Result<MangaFiles, DieselError> {
        use schema::manga_files::dsl::*;

        diesel::insert_into(manga_files::table)
            .values(&manga_files)
            .get_result::<MangaFiles>(conn)
    }
}

impl Insertable<manga_files::table> for MangaFiles {
    fn values(&self) -> crate::rocket_contrib::databases::diesel::InsertValues<manga_files::table> {
        use crate::rocket_contrib::databases::diesel::dsl::{columns, values};

        values((
            manga_files::manga_id.eq(self.manga_id),
            manga_files::filename.eq(self.filename.clone()),
        ))
    }
}
