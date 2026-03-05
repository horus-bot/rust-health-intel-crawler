pub fn get_feeds() -> Vec<&'static str> {
    vec![
        // Chennai health news
        "https://news.google.com/rss/search?q=health+chennai",

        // Global health news
        "https://news.google.com/rss/search?q=global+health",

        // NYTimes health
        "https://rss.nytimes.com/services/xml/rss/nyt/Health.xml",

        // WHO updates
        "https://www.who.int/rss-feeds/news-english.xml",
    ]
}