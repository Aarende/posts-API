pub mod users_handlers;
pub mod posts_handlers;

pub async fn ping() -> &'static str {
    "pong!"
}
