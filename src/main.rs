use std::env;
use axum::{
    Router,
    routing::{get, post, patch, delete}
};
use jsonwebtoken::crypto::aws_lc;
use posts_api_server::{
    errors::app_errors::AppError,
    handlers::*, state::AppState
};
use dotenv::dotenv;
use tokio::net::TcpListener;

async fn fallback() -> Result<&'static str, AppError> {
    Err(AppError::NotFound)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = argon2_async::Config::new();
    argon2_async::set_config(config).await;
    jsonwebtoken::crypto::CryptoProvider::install_default(&aws_lc::DEFAULT_PROVIDER)
        .expect("failed to set cryptoprovider");

    let state = AppState::for_main();

    let listener = TcpListener::bind (
        env::var("SERVER_IP").expect("SERVER_IP must be set")
    )
        .await
        .expect("failed to open connection");

    println!("Сервер открыл поделючение!");

    let app = Router::new()
        .route("/", get(ping))
        .route("/users/id/{id}", get(users_handlers::get_user_by_id))
        .route("/users/nickname/{nickname}", get(users_handlers::get_user_by_nickname))
        .route("/users", get(users_handlers::get_users_with_sort))
        .route("/users", post(users_handlers::post_user))
        .route("/users", patch(users_handlers::patch_user))
        .route("/login", post(users_handlers::sign_in))
        .route("/logout", delete(users_handlers::logout))
        .route("/refresh", get(users_handlers::refresh))
        .route("/posts/id/{id}", get(posts_handlers::get_post_by_id))
        .route("/posts/author/{author}", get(posts_handlers::get_posts_from_author))
        .route("/posts", get(posts_handlers::get_posts_with_sort))
        .route("/posts", post(posts_handlers::post_post))
        .route("/posts", patch(posts_handlers::patch_post))
        .fallback(fallback)
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}
