use std::sync::{Arc, Mutex};

use altcdp::{
    index, inscreva_se, logout, oficina_detail, oficinas_preview, verifica_login, AppState,
};
use axum::routing::{get, post, Router};
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() {
    let pool = Pool::<Postgres>::connect("postgres://postgres:1234@localhost:5432/postgres")
        .await
        .unwrap();

    let state = AppState {
        db: pool,
        login: Arc::new(Mutex::new(false)),
    };
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("debug,hyper=info,mio=info"))
    }
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(index))
        .route("/oficinas", get(oficinas_preview))
        .route("/oficinas/:id", get(oficina_detail))
        .route("/inscreva_se", get(inscreva_se))
        .route("/inscreva_se", post(verifica_login))
        .route("/logout", get(logout))
        .with_state(state);
    println!("Backend listening at 0.0.0.0:8081");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
