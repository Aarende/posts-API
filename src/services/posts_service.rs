use validator::Validate;
use crate::{
    errors::app_errors::AppError,
    models::posts::{GetPost, PostPost, PatchPost},
    repositories::posts_repositories::{self, SortPosts},
    state::{AppState, jwt::AccessClaims}
};

pub async fn get_post_by_id(state: AppState, post_id: i32) -> Result<GetPost, AppError> {
    let mut connection = state.pool.get().await
        .map_err(|_| AppError::Internal)?;

    Ok(posts_repositories::get_post_by_id(&mut connection, post_id).await?)
}

pub async fn get_posts_from_author(state: AppState, offset: i64, author: &str) -> Result<Vec<GetPost>, AppError> {
    let mut connection = state.pool.get().await
        .map_err(|_| AppError::Internal)?;

    Ok(posts_repositories::get_posts_from_author(&mut connection, offset, author).await?)
}

pub async fn get_posts_with_sort(state: AppState, offset: i64, criteria: SortPosts) -> Result<Vec<GetPost>, AppError> {
    let mut connection = state.pool.get().await
        .map_err(|_| AppError::Internal)?;

    Ok(posts_repositories::get_posts_with_sort(&mut connection, offset, criteria).await?)
}

pub async fn post_post(state: AppState, post: PostPost, claims: AccessClaims) -> Result<GetPost, AppError> {
    claims.validate_time()?;
    if post.validate().is_err() {
        return Err(AppError::BadRequest);
    }
    if claims.nickname != post.author {
        return Err(AppError::Forbidden);
    }

    let post = post.to_db_variant();

    let mut connection = state.pool.get()
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(posts_repositories::post_post(&mut connection, &post).await?)
}

pub async fn patch_post(state: AppState, post: PatchPost, claims: AccessClaims) -> Result<GetPost, AppError> {
    claims.validate_time()?;
    if post.validate().is_err() {
        return Err(AppError::BadRequest);
    }
    if claims.nickname != post.author {
        return Err(AppError::Forbidden);
    }

    let mut connection = state.pool.get()
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(posts_repositories::patch_post(&mut connection, &post).await?)
}
