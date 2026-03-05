mod crawler;
mod feeds;
mod models;

use crawler::fetch_feed;
use feeds::get_feeds;
use futures::future::join_all;

#[tokio::main]
async fn main() {

    let feeds = get_feeds();

    let tasks: Vec<_> = feeds
        .iter()
        .map(|url| tokio::spawn(fetch_feed(url)))
        .collect();

    let results = join_all(tasks).await;

    for result in results {

        if let Ok(articles) = result {

            for article in articles {
                println!("Title: {}", article.title);
                println!("Link: {}", article.link);
                println!("Published: {:?}", article.published);
                println!("--------------------------------");
            }

        }
    }
}