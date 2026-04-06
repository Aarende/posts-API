use diesel_async::{AsyncConnection, scoped_futures::ScopedFutureExt};
use validator::Validate;
use crate::{
    errors::app_errors::AppError,
    models::users::{GetUser, PatchUser},
    repositories::users_repositories::{self, SortUser},
    repositories::posts_repositories::change_author_for_posts,
    state::{AppState, jwt::AccessClaims}
};

pub async fn get_user_by_id(state: AppState, user_id: i32) -> Result<GetUser, AppError> {
    let mut connection = state.pool.get()
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(users_repositories::get_user_by_id(&mut connection, user_id).await?)
}

pub async fn get_user_by_nickname(state: AppState, nickname: &str) -> Result<GetUser, AppError> {
    let mut connection = state.pool.get()
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(users_repositories::get_user_by_nickname(&mut connection, nickname).await?)
}

pub async fn get_users_with_sort(state: AppState, offset: i64, criteria: SortUser) -> Result<Vec<GetUser>, AppError> {
    let mut connection = state.pool.get()
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(users_repositories::get_users_with_sort(&mut connection, offset, criteria).await?)
}

pub async fn patch_user(state: AppState, mut user: PatchUser, claims: AccessClaims) -> Result<GetUser, AppError> {
    claims.validate_time()?;

    let claim_user_id = claims.sub.parse::<i32>()
        .map_err(|_| AppError::Unauthorized)?;

    if user.id != claim_user_id {
        return Err(AppError::Forbidden);
    }

    if user.validate().is_err() {
        return Err(AppError::BadRequest);
    }

    user.add_update_time();

    let mut connection = state.pool.get()
        .await
        .map_err(|_| AppError::Internal)?;

    let patched_user = connection.transaction::<_, AppError, _>(|connection| {
        async {
            let patched_user = users_repositories::patch_user(connection, &user).await?;

            if let Some(old_nickname) = user.nickname && old_nickname != patched_user.nickname {
                change_author_for_posts(connection, &old_nickname, &patched_user.nickname).await?;
            }

            Ok(patched_user)
        }.scope_boxed()
    }).await?;

    Ok(patched_user)
}
