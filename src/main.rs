use altcdp::{index, login, oficina_detail, oficinas_preview, AppState};
use axum::routing::{get, Router};
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() {
    let pool = Pool::<Postgres>::connect("postgres://postgres:1234@localhost:5432/postgres")
        .await
        .unwrap();

    let state = AppState {
        db: pool,
    };
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("debug,hyper=info,mio=info"))
    }
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(index))
        .route("/oficinas", get(oficinas_preview))
        .route("/oficinas/:id", get(oficina_detail))
        .route("/login", get(login))
        .with_state(state);
    println!("Backend listening at 0.0.0.0:8081");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
