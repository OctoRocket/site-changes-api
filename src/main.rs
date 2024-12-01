mod args;

mod github;

use axum::{handler::Handler, routing::get, Router, http::StatusCode};
use clap::Parser;
use github::AstaCommitApi;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    let commit_api = AstaCommitApi::new();

    let route = Router::new().route("/", get(get_commits(Arc::new(Mutex::new(commit_api)))));
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
