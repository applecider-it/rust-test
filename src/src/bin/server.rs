use axum::{routing::get, Router};

use myapp::handlers::root::root_handler;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Webサーバーが起動しました！ http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}