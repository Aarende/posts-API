use diesel_async::{AsyncConnection, scoped_futures::ScopedFutureExt};
use validator::Validate;
use argon2_async::{hash, verify};
use crate::{
    errors::app_errors::AppError,
    models::users::{PostUser, UserVerify},
    repositories::users_repositories::*,
    state::{AppState, jwt::TokenResponse, jwt::RefreshClaims}
};

pub async fn sign_up(state: AppState, user: PostUser) -> Result<TokenResponse, AppError> {
    if user.validate().is_err() {
        return Err(AppError::BadRequest);
    }

    let mut user = user.to_db_variant();

    user.password_hash = hash(user.password_hash)
        .await
        .map_err(|_| AppError::Internal)?;

    let mut connection = state.pool.get()
        .await
        .map_err(|_| AppError::Internal)?;

    let token_response = connection.transaction::<_, AppError, _>(|connection| {
        async {
            let user = post_user(connection, &user).await?;

            let access_token = state.jwt_config.generate_access_token(user.id, &user.nickname)
                .map_err(|_| AppError::Internal)?;

            let refresh_token = state.jwt_config.generate_refresh_token(&state, user.id)
                .await.map_err(|_| AppError::Internal)?;

            Ok(TokenResponse {
                access_token,
                refresh_token,
                user_id: user.id,
                nickname: user.nickname
            })
        }.scope_boxed()
    }).await?;

    Ok(token_response)
}

pub async fn sign_in(state: AppState, user: UserVerify) -> Result<TokenResponse, AppError> {
    let mut connection = state.pool.get()
        .await
        .map_err(|_| AppError::Internal)?;

    let db_user = get_db_user(&mut connection, &user.nickname).await?;

    if !verify(user.password, db_user.password_hash)
        .await
        .map_err(|_| AppError::Internal)?
    {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        return Err(AppError::BadRequest);
    }

    Ok(TokenResponse {
        access_token:
            state.jwt_config.generate_access_token(db_user.id, &db_user.nickname)
            .map_err(|_| AppError::Internal)?,
        refresh_token:
            state.jwt_config.generate_refresh_token(&state, db_user.id)
            .await.map_err(|_| AppError::Internal)?,
        user_id: db_user.id,
        nickname: db_user.nickname
    })
}

pub async fn logout(state: AppState, id: i32) {
    let mut tokens = state.token_storage.lock().await;

    tokens.retain(|_, token| token.user_id != id);
}

pub async fn refresh_tokens(state: AppState, claims: RefreshClaims) -> Result<TokenResponse, AppError> {
    claims.validate_time_and_access(&state).await?;

    let user_id = claims.sub
        .parse::<i32>()
        .map_err(|_| AppError::Internal)?;

    let mut connection = state.pool.get()
        .await
        .map_err(|_| AppError::Internal)?;

    let user = get_user_by_id(&mut connection, user_id)
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let response = TokenResponse {
        access_token: state.jwt_config.generate_access_token(user_id, &user.nickname)
            .map_err(|_| AppError::Internal)?,
        refresh_token: state.jwt_config.generate_refresh_token(&state, user_id)
            .await
            .map_err(|_| AppError::Internal)?,
        user_id,
        nickname: user.nickname.to_string()
    };

    let mut token_storage = state.token_storage.lock().await;
    let _ = token_storage.remove(&claims.jti);

    Ok(response)
}
