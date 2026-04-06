use axum::{Json, debug_handler, extract::{Path, Query, State}};
use axum_jwt_auth::Claims;
use serde::Deserialize;
use crate::{
    errors::app_errors::AppError,
    models::posts::{GetPost, PatchPost, PostPost},
    repositories::posts_repositories::SortPosts,
    services::posts_service,
    state::{AppState, jwt::AccessClaims}
};

#[derive(Deserialize)]
pub struct Pagination {
    sort_by: Option<String>,
    offset: Option<i64>
}

#[debug_handler]
pub async fn get_post_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<GetPost>, AppError> {
    Ok(Json(posts_service::get_post_by_id(state, id).await?))
}

pub async fn get_posts_from_author(State(state): State<AppState>, Path(author): Path<String>, Query(offset): Query<i64>) -> Result<Json<Vec<GetPost>>, AppError> {
    Ok(Json(posts_service::get_posts_from_author(state, offset, &author).await?))
}

pub async fn get_posts_with_sort(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> Result<Json<Vec<GetPost>>, AppError> {
    let criteria = match pagination.sort_by.as_deref() {
        Some("creation_time_asc") => SortPosts::ByCreationTimeASC,
        Some("creation_time_desc") => SortPosts::ByCreationTimeDESC,
        _ => SortPosts::ByCreationTimeDESC
    };
    let offset = pagination.offset.unwrap_or(0).max(0);
    Ok(Json(posts_service::get_posts_with_sort(state, offset, criteria).await?))
}

pub async fn post_post(State(state): State<AppState>, claims: Claims<AccessClaims>, Json(post): Json<PostPost>) -> Result<Json<GetPost>, AppError> {
    Ok(Json(posts_service::post_post(state, post, claims.claims).await?))
}

pub async fn patch_post(State(state): State<AppState>, claims: Claims<AccessClaims>, Json(post): Json<PatchPost>) -> Result<Json<GetPost>, AppError> {
    Ok(Json(posts_service::patch_post(state, post, claims.claims).await?))
}
