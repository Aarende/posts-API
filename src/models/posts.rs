use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use validator::Validate;
use crate::schema::posts;

// через Db вариант происходит взаимодействие с бд

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = posts)]
pub struct GetPost {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub content: String,
    pub author: String,
    pub created_at: NaiveDateTime,
    pub last_updated: NaiveDateTime
}

pub type DbGetPost = GetPost;

#[derive(Clone, Deserialize, Validate)]
pub struct PostPost {
    #[validate(length(min = 3, max = 50))]
    pub title: String,

    #[validate(length(min = 3, max = 200))]
    pub description: Option<String>,

    #[validate(length(min = 20))]
    pub content: String,

    pub author: String
}

impl PostPost {
    pub fn to_db_variant(self) -> DbPostPost {
        let now = Utc::now().naive_local();
        DbPostPost {
            title: self.title,
            description: self.description,
            author: self.author,
            content: self.content,
            created_at: now,
            last_updated: now
        }
    }
}

#[derive(Deserialize, AsChangeset, Validate)]
#[diesel(table_name = posts)]
pub struct PatchPost {
    pub id: i32,

    #[validate(length(min = 3, max = 50))]
    pub title: Option<String>,

    #[validate(length(min = 3, max = 200))]
    pub description: Option<String>,

    #[validate(length(min = 20, max = 50))]
    pub content: Option<String>,

    pub author: String,
    pub last_updated: Option<NaiveDateTime>
}

impl PatchPost {
    pub fn add_update_time(&mut self) {
        self.last_updated = Some(Utc::now().naive_local());
    }
}

pub type DbPatchPost = PatchPost;

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct DbPostPost {
    pub title: String,
    pub description: Option<String>,
    pub content: String,
    pub author: String,
    pub created_at: NaiveDateTime,
    pub last_updated: NaiveDateTime
}
