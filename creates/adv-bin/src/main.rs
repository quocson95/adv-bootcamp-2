use axum::{routing, Router, Server};
use tracing::info;
#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    let subsriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subsriber).expect("set global subscriber failed");
    println!("Hello, world!");
    let app = Router::new().route("/", routing::get(hello));
    let addr = "127.0.0.1:1234";
    info!("Server is running on addr: {}", &addr);

    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn hello() -> String {
    String::from("Hello tokio world")
}
