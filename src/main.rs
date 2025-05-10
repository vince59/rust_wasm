mod controllers;
mod models;
mod utils;
mod views;

use std::sync::{Arc, Mutex};
use axum::{Router, http::header, routing::get, response::IntoResponse};
use minijinja::Environment;
use rusqlite::Connection;

struct AppState {
    env: Environment<'static>,
    db: Arc<Mutex<Connection>>,
}

const WASM_JS: &[u8] = include_bytes!("../../rust_wasm/wasm_client/pkg/wasm_client.js");
const WASM: &[u8] = include_bytes!("../../rust_wasm/wasm_client/pkg/wasm_client_bg.wasm");

async fn serve_wasm_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/javascript")],
        WASM_JS.to_vec(),
    )
}

async fn serve_wasm() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/wasm")],
        WASM.to_vec(),
    )
}

#[tokio::main]
async fn main() {
    let port;
    match utils::get_port_from_args() {
        Ok(p) => port = p,
        Err(err) => {
            eprintln!("{}", err);
            utils::print_usage();
            std::process::exit(1);
        }
    }

    let conn = Arc::new(Mutex::new(
        Connection::open(".\\game.db").expect("❌ Erreur de connexion"),
    ));

    println!("Server starts on port : {port}");
    println!("Type http://localhost:{port} in your browser.");

    let mut env = Environment::new();
    views::template::add_template(&mut env);
    env.add_filter("format_date", utils::format_date);

    let app_state = Arc::new(AppState { env, db: conn });
    let app = Router::new()
        .route("/", get(controllers::home::controller_home))
        .route("/pkg/wasm_client.js", get(serve_wasm_js))
        .route("/pkg/wasm_client_bg.wasm", get(serve_wasm))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
