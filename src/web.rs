use webpage::{Webpage, WebpageOptions};

pub fn title(url: &str) -> Result<String, crate::Error> {
    let info = Webpage::from_url(url, WebpageOptions::default())?;
    Ok(info.html.title.unwrap_or("".to_string()))
}
