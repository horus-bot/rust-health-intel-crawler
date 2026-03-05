use rss::Channel;
use reqwest;
use crate::models::Article;

pub async fn fetch_feed(url: &str) -> Vec<Article> {

    let mut articles = Vec::new();

    let response = match reqwest::get(url).await {
        Ok(resp) => resp.bytes().await.unwrap(),
        Err(_) => return articles,
    };

    let channel = match Channel::read_from(&response[..]) {
        Ok(ch) => ch,
        Err(_) => return articles,
    };

    for item in channel.items() {

        let article = Article {
            title: item.title().unwrap_or("").to_string(),
            link: item.link().unwrap_or("").to_string(),
            published: item.pub_date().map(|s| s.to_string()),
        };

        articles.push(article);
    }

    articles
}