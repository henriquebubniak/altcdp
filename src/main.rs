use altcdp::{
    criar_usuario, index, inscreva_se, login, logout, oficina_detail, oficinas_preview, presenca,
    verifica_login, AppState,
};
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, post, Router},
    BoxError,
};
use sqlx::{Pool, Postgres};
use time::Duration;
use tower::ServiceBuilder;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

#[tokio::main]
async fn main() {
    let pool = Pool::<Postgres>::connect("postgres://postgres:1234@localhost:5432/postgres")
        .await
        .unwrap();

    let session_store = MemoryStore::default();
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(
            SessionManagerLayer::new(session_store)
                .with_secure(false)
                .with_expiry(Expiry::OnInactivity(Duration::hours(2))),
        );

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
        .layer(session_service)
        .with_state(state);
    println!("Backend listening at 0.0.0.0:8081");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
