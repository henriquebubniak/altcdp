use altcdp::{handlers::*, AppState};
use axum::routing::{get, post, Router};
use sqlx::{Pool, Postgres};
use time::Duration;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

#[tokio::main]
async fn main() {
    let pool = Pool::<Postgres>::connect("postgres://postgres:1234@localhost:5432/postgres")
        .await
        .unwrap();

    let session_store = MemoryStore::default();
    let session_expiry = Expiry::OnInactivity(Duration::hours(1));
    let session_service = SessionManagerLayer::new(session_store)
        .with_name("user")
        .with_secure(false)
        .with_expiry(session_expiry);

    let state = AppState { db: pool };
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug,hyper=info,mio=info")
    }
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(index))
        .route("/oficinas", get(oficinas_preview))
        .route("/oficinas/:id", get(oficina_detail))
        .route("/oficinas/:id", post(presenca))
        .route("/login", get(login))
        .route("/login", post(verifica_login))
        .route("/logout", get(logout))
        .route("/inscreva_se", get(inscreva_se))
        .route("/inscreva_se", post(criar_usuario))
        .route("/perfil", get(perfil))
        .route("/criar_oficina", get(criar_oficina))
        .route("/criar_oficina", post(criar_oficina_form))
        .layer(session_service)
        .with_state(state);
    println!("Backend listening at 0.0.0.0:8081");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
