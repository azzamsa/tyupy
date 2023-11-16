use reqwest::Client;
use select::{document::Document, predicate::Name};
use url::Url;

pub async fn title(url: &Url) -> Result<String, crate::Error> {
    let response = Client::new().get(url.to_string()).send().await?;
    let body = response.text().await?;

    let document = Document::from_read(body.as_bytes())?;

    let title = document
        .find(Name("title"))
        .next()
        .map(|title_node| title_node.text())
        .unwrap_or_else(|| "No title found".to_string());

    Ok(title)
}
