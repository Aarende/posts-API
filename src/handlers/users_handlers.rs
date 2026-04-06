use axum::{Json, extract::{Path, Query, State}};
use axum_jwt_auth::Claims;
use serde::Deserialize;
use crate::{
    errors::app_errors::AppError,
    models::users::{GetUser, PatchUser, PostUser, UserVerify},
    repositories::users_repositories::SortUser,
    services::{authentification, users_service},
    state::{AppState, jwt::{AccessClaims, RefreshClaims, TokenResponse}}
};

#[derive(Deserialize)]
pub struct Pagination {
    sort_by: Option<String>,
    offset: Option<i64>
}

pub async fn get_user_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<GetUser>, AppError> {
    Ok(Json(users_service::get_user_by_id(state, id).await?))
}

pub async fn get_user_by_nickname(State(state): State<AppState>, Path(nickname): Path<String>) -> Result<Json<GetUser>, AppError> {
    Ok(Json(users_service::get_user_by_nickname(state, &nickname).await?))
}

pub async fn get_users_with_sort(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> Result<Json<Vec<GetUser>>, AppError> {
    let criteria = match pagination.sort_by.as_deref() {
        Some("nickname_asc") => SortUser::ByNicknameASC,
        Some("nickname_desc") => SortUser::ByNicknameDESC,
        Some("creation_time_asc") => SortUser::ByCreationTimeASC,
        Some("creation_time_desc") => SortUser::ByCreationTimeDESC,
        _ => SortUser::ByCreationTimeDESC
    };
    let offset = pagination.offset.unwrap_or(0).max(0);
    Ok(Json(users_service::get_users_with_sort(state, offset, criteria).await?))
}

pub async fn post_user(State(state): State<AppState>, Json(user): Json<PostUser>) -> Result<Json<TokenResponse>, AppError> {
    Ok(Json(authentification::sign_up(state, user).await?))
}

pub async fn patch_user(State(state): State<AppState>, claims: Claims<AccessClaims>, Json(user): Json<PatchUser>) -> Result<Json<GetUser>, AppError> {
    Ok(Json(users_service::patch_user(state, user, claims.claims).await?))
}

pub async fn sign_in(State(state): State<AppState>, Json(user): Json<UserVerify>) -> Result<Json<TokenResponse>, AppError> {
    Ok(Json(authentification::sign_in(state, user).await?))
}

pub async fn logout(State(state): State<AppState>, claims: Claims<RefreshClaims>) {
    authentification::logout(state, claims.claims.sub.parse::<i32>().unwrap_or(0)).await
}

pub async fn refresh(State(state): State<AppState>, claims: Claims<RefreshClaims>) -> Result<Json<TokenResponse>, AppError> {
    Ok(Json(authentification::refresh_tokens(state, claims.claims).await?))
}
