use std::{collections::HashMap, sync::Arc};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, encode};
use axum_jwt_auth::{LocalDecoder, Decoder};
use serde::{Deserialize, Serialize};
use crate::{constants::*, errors::app_errors::AppError};
use super::AppState;

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: i32,
    pub nickname: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub nickname: String
}

impl AccessClaims {
    pub fn validate_time(&self) -> Result<(), AppError> {
        let now = Utc::now().timestamp() as usize;
        if self.iat > now || self.exp < now {
            return Err(AppError::Unauthorized);
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
}

impl RefreshClaims {
    pub async fn validate_time_and_access(&self, state: &AppState) -> Result<(), AppError> {
        let now = Utc::now().timestamp() as usize;
        if self.iat > now || self.exp < now {
            return Err(AppError::Unauthorized);
        }
        let token_storage = state.token_storage.lock().await;
        let info = token_storage.get(&self.jti);
        match info {
            Some(info) => {
                let user_id = self.sub.parse::<i32>()
                    .map_err(|_| AppError::Unauthorized)?;
                if !(info.created_at == self.iat && info.expires_at == self.exp && info.user_id == user_id) {
                    return Err(AppError::Unauthorized);
                }
            }
            None => return Err(AppError::Unauthorized)
        }
        Ok(())
    }
}

pub struct TokenInfo {
    pub user_id: i32,
    pub created_at: usize,
    pub expires_at: usize,
}

pub type TokenStorage = HashMap<String, TokenInfo>;

pub struct JwtConfig {
    pub audience: String,
    pub access_secret: String,
    pub refresh_secret: String,
    pub access_expiration_minutes: i64,
    pub refresh_expiration_days: i64,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        Self {
            audience: std::env::var("AUDIENCE")
                .expect("AUDIENCE must be set"),
            access_secret: std::env::var("JWT_ACCESS_SECRET")
                .expect("JWT_ACCESS_SECRET must be set"),
            refresh_secret: std::env::var("JWT_REFRESH_SECRET")
                .expect("JWT_REFRESH_SECRET must be set"),
            access_expiration_minutes: ACCESS_EXPIRATION_MINUTES,
            refresh_expiration_days: REFRESH_EXPIRATION_DAYS,
        }
    }
    pub fn generate_access_token(&self, user_id: i32, nickname: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = now + Duration::minutes(self.access_expiration_minutes);

        let claims = AccessClaims {
            sub: user_id.to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            nickname: nickname.to_string()
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.access_secret.as_bytes())
        )
    }
    pub async fn generate_refresh_token(&self, state: &AppState, user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = now + Duration::days(self.refresh_expiration_days);

        let claims = RefreshClaims {
            sub: user_id.to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            jti: uuid::Uuid::new_v4().to_string()
        };

        let mut token_storage = state.token_storage.lock().await;
        token_storage.insert(claims.jti.clone(), TokenInfo {
            user_id,
            created_at: claims.iat,
            expires_at: claims.exp
        });

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.refresh_secret.as_bytes())
        )
    }
    pub fn create_access_decoder(&self) -> Decoder<AccessClaims> {
        let keys = vec![DecodingKey::from_secret(self.access_secret.as_bytes())];

        let mut validation = Validation::default();
        validation.set_audience(&[&self.audience]);
        
        let local_decoder = LocalDecoder::builder()
            .keys(keys)
            .validation(validation)
            .build()
            .expect("Failed to create decoder");

        Arc::new(local_decoder)
    }
    pub fn create_refresh_decoder(&self) -> Decoder<RefreshClaims> {
        let keys = vec![DecodingKey::from_secret(self.refresh_secret.as_bytes())];

        let mut validation = Validation::default();
        validation.set_audience(&[&self.audience]);
        
        let local_decoder = LocalDecoder::builder()
            .keys(keys)
            .validation(validation)
            .build()
            .expect("Failed to create decoder");

        Arc::new(local_decoder)
    }
}
