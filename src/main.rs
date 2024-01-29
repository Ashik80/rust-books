use std::{sync::Arc, env};
use axum::Router;
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;
use crate::contacts::routes::get_routes;

mod config;
mod contacts;

pub struct AppState {
    pool: PgPool
}

pub type ApiState = Arc<AppState>;

#[tokio::main]
async fn main() {
    dotenv().expect("failed to load env");

    let database_url = env::var("DATABASE_URL").expect("failed to get database url");
    let pool = PgPoolOptions::new().connect(&database_url).await.expect("failed to connect to database");
    let app_state = Arc::new(AppState { pool });

    let routes = Router::new().nest("/contacts", get_routes()).with_state(app_state);
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("error binding address");

    println!("Listening on {}", listener.local_addr().expect("error getting address"));
    axum::serve(listener, routes).await.expect("error serving app");
}
