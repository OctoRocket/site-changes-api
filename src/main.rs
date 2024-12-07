mod args;

mod github;

use axum::{handler::Handler, routing::get, Router, http::StatusCode};
use clap::Parser;
use github::AstaCommitApi;
use http::Method;
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    let commit_api = AstaCommitApi::new();

    let cors = CorsLayer::new()    // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(cors::Any);

    let middleware = ServiceBuilder::new().layer(cors);
    let route = Router::new()
        .route("/", get(get_commits(Arc::new(Mutex::new(commit_api)))))
        .layer(middleware);

    let listener = match TcpListener::bind(format!("127.0.0.1:{}", args.port)).await {
        Ok(v) => v,
        Err(_) => {
            println!("Failed to bind to port {}", args.port);
            return;
        },
    };

    axum::serve(listener, route).await.unwrap();
}

fn get_commits(commits: Arc<Mutex<AstaCommitApi>>) -> impl Handler<((),), ()> {
    || async move {
        match commits.lock().await.get().await {
            Ok(r) => Ok(serde_json::to_string(&r).unwrap()),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
