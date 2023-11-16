use reqwest::Client;
use select::document::Document;
use select::predicate::Name;

pub async fn title(url: &str) -> Result<String, crate::Error> {
    let response = Client::new().get(url).send().await?;
    let body = response.text().await?;

    let document = Document::from_read(body.as_bytes())?;

    let title = document
        .find(Name("title"))
        .next()
        .map(|title_node| title_node.text())
        .unwrap_or_else(|| "No title found".to_string());

    Ok(title)
}
