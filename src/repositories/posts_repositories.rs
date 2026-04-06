use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use super::PgConnection;
use crate::{
    schema::posts,
    models::posts::*,
    errors::db_errors::*,
    constants::LIMIT_OF_QUERY_RESULTS
};

pub enum SortPosts {
    ByCreationTimeASC,
    ByCreationTimeDESC
}

pub async fn get_post_by_id(connection: &mut PgConnection, post_id: i32) -> Result<DbGetPost, GetError> {
    posts::table
        .find(post_id)
        .select(DbGetPost::as_select())
        .first::<DbGetPost>(connection)
        .await
        .map_err(|_| GetError::NotFound)
}

pub async fn get_posts_from_author(connection: &mut PgConnection, offset: i64, author: &str) -> Result<Vec<DbGetPost>, GetError> {
    posts::table
        .filter(posts::author.eq(author))
        .limit(LIMIT_OF_QUERY_RESULTS)
        .offset(offset)
        .select(DbGetPost::as_select())
        .load::<DbGetPost>(connection)
        .await
        .map_err(|_| GetError::Internal)
}

pub async fn get_posts_with_sort(connection: &mut PgConnection, offset: i64, sort: SortPosts) -> Result<Vec<DbGetPost>, GetError> {
    match sort {
        SortPosts::ByCreationTimeASC => {
            posts::table
                .order_by(posts::id.asc())
                .limit(LIMIT_OF_QUERY_RESULTS)
                .offset(offset)
                .select(DbGetPost::as_select())
                .load::<DbGetPost>(connection)
                .await
                .map_err(|_| GetError::Internal)
        }
        SortPosts::ByCreationTimeDESC => {
            posts::table
                .order_by(posts::id.desc())
                .limit(LIMIT_OF_QUERY_RESULTS)
                .offset(offset)
                .select(DbGetPost::as_select())
                .load::<DbGetPost>(connection)
                .await
                .map_err(|_| GetError::Internal)
        }
    }
}

pub async fn post_post(connection: &mut PgConnection, post: &DbPostPost) -> Result<DbGetPost, PostError> {
    diesel::insert_into(posts::table)
        .values(post)
        .returning(DbGetPost::as_returning())
        .get_result::<DbGetPost>(connection)
        .await
        .map_err(|_| PostError::Internal)
}

pub async fn patch_post(connection: &mut PgConnection, post: &DbPatchPost) -> Result<DbGetPost, PatchError> {
    diesel::update(posts::table.find(post.id))
        .filter(posts::author.eq(&post.author))
        .set(post)
        .returning(DbGetPost::as_returning())
        .get_result::<DbGetPost>(connection)
        .await
        .map_err(|_| PatchError::Internal)
}

pub async fn change_author_for_posts(connection: &mut PgConnection, prev_nickname: &str, new_nickname: &str) -> Result<(), PatchError> {
    match diesel::update(posts::table)
        .filter(posts::author.eq(prev_nickname))
        .set(posts::author.eq(new_nickname))
        .execute(connection)
        .await
    {
        Ok(0) => Err(PatchError::NotFound),
        Err(_) => Err(PatchError::Internal),
        _ => Ok(())
    }
}
