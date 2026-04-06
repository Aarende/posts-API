pub mod jwt;

use axum::extract::FromRef;
use axum_jwt_auth::Decoder;
use jwt::*;
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;
use crate::repositories::{PgPool, create_pool};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub jwt_config: Arc<JwtConfig>,
    pub token_storage: Arc<Mutex<TokenStorage>>,
    pub pool: PgPool,
    pub access_decoder: Decoder<AccessClaims>,
    pub refresh_decoder: Decoder<RefreshClaims>
}

impl AppState {
    pub fn for_main() -> Self {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let config = JwtConfig::from_env();
        let access_decoder = config.create_access_decoder();
        let refresh_decoder = config.create_refresh_decoder();
        AppState {
            jwt_config: Arc::new(config),
            token_storage: Arc::new(Mutex::new(HashMap::new())),
            pool: create_pool(&database_url),
            access_decoder,
            refresh_decoder
        }
    }
}
