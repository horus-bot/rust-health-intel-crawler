Health Crawler
High-Performance Async Web Scraper in Rust

Author: Harsh
GitHub: horus-bot
Overview

Health Crawler is a high-performance asynchronous web crawler written in Rust designed to ingest RSS feeds, extract article content, and expose structured data for downstream processing.

The project was initially built as an experiment to evaluate Rust's performance for high-concurrency web scraping workloads compared to higher-level languages.

What started as a small experiment quickly demonstrated something interesting:

Rust can process large numbers of concurrent HTTP requests and HTML parsing tasks with significantly lower overhead than typical scripting-language implementations.

The crawler currently focuses on health news aggregation, but the architecture is generic and can be adapted to scrape any topic or website ecosystem.

Key Features

• Async RSS ingestion
• Concurrent HTTP crawling
• Automatic article text extraction
• Recent-article filtering
• Clean structured output
• Easily configurable feed sources
• Designed for high-throughput workloads

Architecture

The crawler follows a simple but efficient pipeline.

What is this?
How It Works

RSS feeds are loaded from feeds.rs

The crawler fetches each feed asynchronously

Feed items are parsed

Article URLs are extracted

Articles are fetched concurrently using reqwest

HTML content is parsed using scraper

Relevant text paragraphs are extracted

Results are structured as Article objects

Project Structure
│
├── src/
│   ├── main.rs        # program entry
│   ├── crawler.rs     # scraping + parsing logic
│   ├── feeds.rs       # RSS feed configuration
│   └── models.rs      # data structures
│
├── Cargo.toml
└── README.md
Installation

Install Rust:

https://rustup.rs

Clone repository:

git clone https://github.com/horus-bot/health_crawler
cd health_crawler


cargo build --release

Run crawler:

cargo run --release

Development mode:

cargo run
Customizing the Crawler

The crawler can easily be adapted for any scraping task.

Add RSS Sources


src/feeds.rs

Example:

"https://news.google.com/rss/search?q=health+chennai"
"https://rss.nytimes.com/services/xml/rss/nyt/Health.xml"

You can replace these with feeds for:

• finance
• technology
• research papers
• cybersecurity
• sports

Change Scraping Logic

Inside:

src/crawler.rs

The scraper currently extracts:

Selector::parse("p")
You can adapt this for specific sites:

article p
.post-content p
#main-content p
Control Extraction Volume

Modify:

.take(30)

to increase or reduce paragraph extraction.

You can also adjust the filter:

text.len() > 40
Performance Motivation

The main motivation behind this project was to compare Rust vs higher-level languages for web crawling workloads.

The same crawler architecture was initially tested using Python.


Language Comparison

The following graph illustrates a typical throughput comparison for concurrent HTTP workloads.

What is this?
Throughput Comparison (Conceptual)
Diagram is not supported.
What is this?

Note:

These are conceptual comparisons based on typical behavior of async IO workloads. Real numbers vary depending on:

• network latency
• server rate limiting
• HTML complexity
• machine hardware

Why Rust Performs Well Here

Rust combines several properties that make it excellent for scraping workloads:

Native Performance
Rust compiles to native machine code with zero runtime overhead.

Efficient Async Runtime

The Tokio runtime allows thousands of concurrent tasks with minimal overhead.

Low Memory Usage

Each async task has extremely small memory footprint compared to typical thread-based approaches.

Zero-Cost Abstractions

Rust's abstractions compile down to efficient machine instructions.

Benchmarking Your Setup

To benchmark your own environment:
Collect ~100 article URLs

Run the crawler with different concurrency levels

Measure total execution time

Example tool:

hyperfine

Example:

hyperfine "cargo run --release"
Extending the Project

Possible improvements:

Domain-specific selectors

Map domains to optimized extraction rules.

Readability algorithms

Use readability-style extraction for cleaner article bodies.

Rate limiting

Add per-host concurrency limits.

Caching

Avoid re-fetching previously processed articles.

Database storage

Persist articles to:

• PostgreSQL
• Elasticsearch
• Redis

Ethics & Responsible Crawling

Always respect:

• robots.txt
• website terms of service
• rate limits

Avoid scraping protected or private content.

License

MIT

Author

Harsh
GitHub: horus-bot