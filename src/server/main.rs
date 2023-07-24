mod handlers;

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Server;
use socketioxide::{Namespace, SocketIoLayer};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use tokio::spawn;
use tokio_schedule::{every, Job};

async fn serve_html() -> impl IntoResponse {
    Html(include_str!("mock_client.html"))
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting server");
    let ns = 
        Namespace::builder().add("/", handlers::handler).build();

    let sioLayer = SocketIoLayer::new(ns);

    let app = axum::Router::new()
        .route("/", get(serve_html))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive()) // Enable CORS policy
                .layer(sioLayer),
        );

    // let debug_timer = every(5).seconds() // by default chrono::Local timezone
    //     .perform(|| async {
    //          println!("{}", sioLayer.) }
    //     );
    // spawn(debug_timer);

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}