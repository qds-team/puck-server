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
}

#[derive(Insertable)]
#[table_name="mangas"]
pub struct NewManga {
    pub name: String,
    pub path: String,
}

impl Manga {
    pub fn all(conn: &SqliteConnection) -> Vec<Manga> {
        use crate::schema::mangas::dsl::*;

        mangas.load::<Manga>(conn).unwrap()
    }

    pub fn get(conn: &SqliteConnection, id: i32) -> Result<Manga, String> {
        use crate::schema::mangas::dsl::*;

        mangas.find(id).first::<Manga>(conn).map_err(|e| e.to_string())
    }

    pub fn insert(conn: &SqliteConnection, manga: NewManga) -> Result<Manga, String> {
        use crate::schema::mangas::dsl::*;

        diesel::insert_into(mangas).values(&manga).execute(conn).map_err(|e| e.to_string())?;

        mangas.order(id.desc()).first::<Manga>(conn).map_err(|e| e.to_string())
    }

    pub fn update(conn: &SqliteConnection, id: i32, manga: NewManga) -> Result<(), String> {
        use crate::schema::mangas::dsl::*;

        diesel::update(mangas.find(id)).set(&manga).execute(conn).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn delete(conn: &SqliteConnection, id: i32) -> Result<(), String> {
        use crate::schema::mangas::dsl::*;

        diesel::delete(mangas.find(id)).execute(conn).map_err(|e| e.to_string())?;

        Ok(())
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