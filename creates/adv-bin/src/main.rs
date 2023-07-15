use axum::{routing, Router, Server};
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = Router::new().route("/", routing::get(hello));

    let addr = "127.0.0.0.1:8888";
    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn hello() -> String {
    String::from("Hello tokio world")
}
