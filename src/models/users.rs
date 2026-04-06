use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use crate::schema::users;
use validator::Validate;

// через Db вариант происходит взаимодействие с бд

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct DbUser {
    pub id: i32,
    pub nickname: String,
    pub password_hash: String,
    pub about: Option<String>,
    pub created_at: NaiveDateTime,
    pub last_updated: NaiveDateTime
}

impl DbUser {
    pub fn to_responce_variant(self) -> GetUser {
        GetUser {
            id: self.id,
            nickname: self.nickname,
            about: self.about,
            created_at: self.created_at,
            last_updated: self.last_updated
        }
    }
}

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct GetUser {
    pub id: i32,
    pub nickname: String,
    pub about: Option<String>,
    pub created_at: NaiveDateTime,
    pub last_updated: NaiveDateTime
}

pub type DbGetUser = GetUser;

#[derive(Deserialize, Validate)]
pub struct PostUser {
    #[validate(length(min = 3, max = 50))]
    pub nickname: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(length(max = 200))]
    pub about: Option<String>
}

impl PostUser {
    pub fn to_db_variant(self) -> DbPostUser {
        let now = Utc::now().naive_local();
        DbPostUser {
            nickname: self.nickname,
            password_hash: self.password,
            about: self.about,
            created_at: now,
            last_updated: now
        }
    }
}

#[derive(Deserialize, AsChangeset, Validate)]
#[diesel(table_name = users)]
pub struct PatchUser {
    pub id: i32,

    #[validate(length(min = 3, max = 50))]
    pub nickname: Option<String>,

    #[validate(length(max = 200))]
    pub about: Option<String>,

    pub last_updated: Option<NaiveDateTime>
}

impl PatchUser {
    pub fn add_update_time(&mut self) {
        self.last_updated = Some(Utc::now().naive_local());
    }
}

pub type DbPatchUser = PatchUser;

#[derive(Deserialize)]
pub struct UserVerify {
    pub nickname: String,
    pub password: String 
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct DbPostUser {
    pub nickname: String,
    pub password_hash: String,
    pub about: Option<String>,
    pub created_at: NaiveDateTime,
    pub last_updated: NaiveDateTime
}
