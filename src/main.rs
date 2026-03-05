mod crawler;
mod feeds;
mod models;

use axum::{Router, routing::get, Json};
use tokio::net::TcpListener;
use futures::future::join_all;

use crawler::fetch_feed;
use feeds::{chennai_feeds, global_feeds};
use models::Article;

async fn chennai_health() -> Json<Vec<Article>> {

    let feeds = chennai_feeds();

    let tasks: Vec<_> = feeds.iter().map(|url| fetch_feed(url)).collect();

    let results = join_all(tasks).await;

    let mut articles = Vec::new();

    for res in results {
        articles.extend(res);
    }

    Json(articles)
}

async fn global_health() -> Json<Vec<Article>> {

    let feeds = global_feeds();

    let tasks: Vec<_> = feeds.iter().map(|url| fetch_feed(url)).collect();

    let results = join_all(tasks).await;

    let mut articles = Vec::new();

    for res in results {
        articles.extend(res);
    }

    Json(articles)
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/chennai-health", get(chennai_health))
        .route("/global-health", get(global_health));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}