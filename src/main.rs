mod controllers;
mod models;
mod utils;
mod views;

use std::sync::{Arc, Mutex};
use axum::{Router, routing::get};
use minijinja::Environment;
use rusqlite::Connection;


struct AppState {
    env: Environment<'static>,
    db: Arc<Mutex<Connection>>,
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
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
