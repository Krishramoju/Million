use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Source: BBC World News RSS feed via rss2json
    let url = "https://api.rss2json.com/v1/api.json?rss_url=https://feeds.bbci.co.uk/news/world/rss.xml";

    // Make the HTTP request
    let response = reqwest::get(url).await?.json::<ApiResponse>().await?;

    // Extract top 5 items
    let top_news: Vec<NewsItem> = response
        .items
        .into_iter()
        .take(5)
        .map(|item| NewsItem {
            title: item.title,
            description: item.description,
        })
        .collect();

    // Wrap it into final JSON structure
    let output = serde_json::json!({ "news": top_news });

    // Write to file
    let mut file = File::create("data.json")?;
    file.write_all(serde_json::to_string_pretty(&output)?.as_bytes())?;

    println!("✅ News saved to data.json");
    Ok(())
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    title: String,
    description: String,
}

#[derive(Debug, Serialize)]
struct NewsItem {
    title: String,
    description: String,
}
