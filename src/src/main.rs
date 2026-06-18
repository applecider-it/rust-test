use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // ① ルーティングの設定（「/」にアクセスしたら「Hello from WSL!」を返す）
    let app = Router::new().route("/", get(|| async { "Hello from WSL! 🚀" }));

    // ② サーバーを起動するアドレス（ポート3000番）
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Webサーバーが起動しました！ http://localhost:3000");

    // ③ サーバーをスタート
    axum::serve(listener, app).await.unwrap();
}