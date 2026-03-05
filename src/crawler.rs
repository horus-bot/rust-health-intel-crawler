use rss::Channel;
use scraper::{Html, Selector};
use chrono::{DateTime, Utc, Duration};

use crate::models::Article;



// ------------------------------------------------
// Check if article is within last 7 days
// ------------------------------------------------
fn is_within_last_week(pub_date: Option<&str>) -> bool {

    if let Some(date_str) = pub_date {

        if let Ok(parsed) = DateTime::parse_from_rfc2822(date_str) {

            let article_time = parsed.with_timezone(&Utc);
            let now = Utc::now();

            return now - article_time <= Duration::days(7);
        }
    }

    false
}



// ------------------------------------------------
// Scrape article content from webpage
// ------------------------------------------------
async fn scrape_article(url: &str) -> Option<String> {

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .ok()?;


    let response = client.get(url).send().await.ok()?;

    let body = response.text().await.ok()?;


    let document = Html::parse_document(&body);

    let selector = Selector::parse("p").ok()?;


    let mut content = String::new();

    for element in document.select(&selector).take(25) {

        let text = element.text().collect::<Vec<_>>().join(" ");

        if text.len() > 40 {
            content.push_str(&text);
            content.push('\n');
        }
    }

    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}



// ------------------------------------------------
// Fetch and parse RSS feed
// ------------------------------------------------
pub async fn fetch_feed(url: &str) -> Vec<Article> {

    let mut articles = Vec::new();


    let response = match reqwest::get(url).await {
        Ok(resp) => match resp.bytes().await {
            Ok(bytes) => bytes,
            Err(_) => return articles,
        },
        Err(_) => return articles,
    };


    let channel = match Channel::read_from(&response[..]) {
        Ok(channel) => channel,
        Err(_) => return articles,
    };


    for item in channel.items() {

        // Filter older than 7 days
        if !is_within_last_week(item.pub_date()) {
            continue;
        }


        let link = item.link().unwrap_or("").to_string();


        let content = scrape_article(&link).await;


        let article = Article {
            title: item.title().unwrap_or("").to_string(),
            link,
            published: item.pub_date().map(|s| s.to_string()),
            content,
        };


        articles.push(article);
    }


    articles
}