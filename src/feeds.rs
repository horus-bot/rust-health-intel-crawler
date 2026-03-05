pub fn chennai_feeds() -> Vec<&'static str> {
    vec![
        // Google News queries
        "https://news.google.com/rss/search?q=health+chennai+when:7d",
        "https://news.google.com/rss/search?q=chennai+hospital+when:7d",
        "https://news.google.com/rss/search?q=chennai+disease+when:7d",
        "https://news.google.com/rss/search?q=chennai+medical+when:7d",

        // Tamil Nadu health coverage
        "https://news.google.com/rss/search?q=tamil+nadu+health+when:7d",
        "https://news.google.com/rss/search?q=tamil+nadu+hospital+when:7d",

        // India health news (regional signals)
        "https://news.google.com/rss/search?q=india+health+when:7d",
    ]
}


pub fn global_feeds() -> Vec<&'static str> {
    vec![
        // Global health agencies
        "https://www.who.int/rss-feeds/news-english.xml",
        "https://tools.cdc.gov/api/v2/resources/media/403372.rss",

        // Major health journalism
        "https://rss.nytimes.com/services/xml/rss/nyt/Health.xml",
        "https://www.theguardian.com/society/health/rss",
        "https://www.reuters.com/world/healthcare-pharmaceuticals/rss",

        // Global health signals
        "https://news.google.com/rss/search?q=global+health+when:7d",
        "https://news.google.com/rss/search?q=disease+outbreak+when:7d",
        "https://news.google.com/rss/search?q=virus+outbreak+when:7d",
        "https://news.google.com/rss/search?q=vaccine+news+when:7d",
        "https://news.google.com/rss/search?q=public+health+when:7d",

        // Medical research news
        "https://www.sciencedaily.com/rss/health_medicine.xml",

        // WHO outbreak signals
        "https://www.who.int/feeds/entity/csr/don/en/rss.xml",
    ]
}