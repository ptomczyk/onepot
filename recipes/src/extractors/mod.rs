use scraper::{Html, Selector};

pub fn extract_hrefs(html: &Html) -> Vec<String> {
    let selector = String::from(".row > .col> a[href^=\"/przepis\"]");
    let selector = Selector::parse(&selector).unwrap();

    html.select(&selector)
        .filter_map(|el| el.value().attr("href"))
        .map(|href| href.to_owned())
        .collect()
}

pub fn extract_name(html: &Html) -> String {
    let selector = String::from(".przepis.page-header");
    let selector = Selector::parse(&selector).unwrap();

    html.select(&selector)
        .map(|el| el.text().collect::<String>())
        .collect()
}

pub fn extract_ingredients(html: &Html) -> Vec<String> {
    let selector = String::from(".field-name-field-skladniki li");
    let selector = Selector::parse(&selector).unwrap();

    html.select(&selector)
        .map(|el| el.text().collect::<String>().trim().to_owned())
        .collect()
}

pub fn extract_preparation(html: &Html) -> Vec<String> {
    let selector = String::from(".field-name-field-przygotowanie li");
    let selector = Selector::parse(&selector).unwrap();

    html.select(&selector)
        .map(|el| el.text().collect::<String>().trim().to_owned())
        .collect()
}