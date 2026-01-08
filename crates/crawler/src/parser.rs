use scraper::{Html, Selector};
use url::Url;

pub fn extract_links(base_url: &str, html_content: &str) -> Vec<String> {
    let document = Html::parse_document(html_content);
    let selector = Selector::parse("a[href]").unwrap();
    let base = Url::parse(base_url).ok();

    document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .filter_map(|href| {
            if let Some(ref base_url) = base {
                base_url.join(href).ok().map(|u| u.to_string())
            } else {
                Some(href.to_string())
            }
        })
        .collect()
}
