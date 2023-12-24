use std::sync::{Arc, Mutex};

use altcdp::{
    index, inscreva_se, logout, oficina_detail, oficinas_preview, verifica_login, AppState,
};
use axum::routing::{get, post, Router};
use clap::Parser;
use sqlx::{Pool, Postgres};

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "0.0.0.0:8081")]
    addr: String,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "../dist")]
    static_dir: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();
    let pool = Pool::<Postgres>::connect("postgres://postgres:1234@localhost:5432/postgres")
        .await
        .unwrap();

    let state = AppState {
        db: pool,
        login: Arc::new(Mutex::new(false)),
    };
    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    // enable console logging
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/oficinas", get(oficinas_preview))
        .route("/oficinas/:id", get(oficina_detail))
        .route("/inscreva_se", get(inscreva_se))
        .route("/inscreva_se", post(verifica_login))
        .route("/logout", get(logout))
        .with_state(state);
    println!("Backend listening at {}", opt.addr);
    let listener = tokio::net::TcpListener::bind(opt.addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
