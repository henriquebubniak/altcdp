use altcdp::{oficina_detail, oficinas, AppState};
use axum::routing::{get, Router};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

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
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:1234@localhost:5432/postgres")
        .await
        .unwrap();

    let state = AppState { db: pool };
    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    // enable console logging
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/oficinas", get(oficinas))
        .route("/oficinas/:id", get(oficina_detail))
        .with_state(state);
    println!("Backend listening at {}", opt.addr);
    let listener = tokio::net::TcpListener::bind(opt.addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
