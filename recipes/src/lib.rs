use rand::seq::SliceRandom;
use reqwest::{header, Client};
use scraper::{Html, Selector};
use url::Url;


use crate::items::{Ingredient, Recipe, Step};

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/onepot.rs"));
}

impl Ingredient {
    fn new(name: String) -> Self {
        let mut ing = Self::default();
        ing.name = name;
        ing
    }
}

impl Step {
    fn new(description: String) -> Self {
        let mut step = Self::default();
        step.description = description;
        step
    }
}

impl Recipe {
    fn new(name: String, ingredients: Vec<String>, preparation: Vec<String>) -> Self {
        let mut rec = Recipe::default();
        rec.name = name;
        rec.ingredients = ingredients
            .into_iter()
            .map(|ingredient| Ingredient::new(ingredient))
            .collect();
        rec.preparation = preparation
            .into_iter()
            .map(|step| Step::new(step))
            .collect();
        rec
    }
}

fn fetch_html(url: &str) -> Html {
    let url = Url::parse(&url).expect("Failed to parse URL");

    let client = Client::new();
    let mut res = client
        .get(url)
        .header(header::ACCEPT, "text/html")
        .send()
        .expect("Failed to send HTTP request");

    let html = res.text().expect("Failed to parse HTML");
    let html = Html::parse_document(&html);

    html
}

fn get_url(endpoint: &str) -> String {
    // let mut rng = rand::thread_rng();
    // let page = rng.gen_range(0, 4);
    format!("https://www.kwestiasmaku.com{}", endpoint)
}

fn extract_hrefs(html: &Html) -> Vec<String> {
    let selector = String::from(".row > .col> a[href^=\"/przepis\"]");
    let selector = Selector::parse(&selector).unwrap();

    html.select(&selector)
        .filter_map(|el| el.value().attr("href"))
        .map(|href| href.to_owned())
        .collect()
}

fn extract_name(html: &Html) -> String {
    let selector = String::from(".przepis.page-header");
    let selector = Selector::parse(&selector).unwrap();

    html.select(&selector)
        .map(|el| el.text().collect::<String>())
        .collect()
}

fn extract_ingredients(html: &Html) -> Vec<String> {
    let selector = String::from(".field-name-field-skladniki li");
    let selector = Selector::parse(&selector).unwrap();

    html.select(&selector)
        .map(|el| el.text().collect::<String>().trim().to_owned())
        .collect()
}

fn extract_preparation(html: &Html) -> Vec<String> {
    let selector = String::from(".field-name-field-przygotowanie li");
    let selector = Selector::parse(&selector).unwrap();

    html.select(&selector)
        .map(|el| el.text().collect::<String>().trim().to_owned())
        .collect()
}

fn extract_links() -> Vec<String> {
    let url = &get_url("/przepisy/jednogarnkowe");
    let html = fetch_html(&url);
    let links = extract_hrefs(&html);
    links
}

fn get_random_link() -> Option<String> {
    let links = extract_links();
    let link = links.choose(&mut rand::thread_rng());
    match link {
        Some(link) => Some(link.to_owned()),
        None => None,
    }
}

pub fn get_recipe() -> Recipe {
    let link = get_random_link().expect("Failed to get random link");
    let link = get_url(&link);
    let html = fetch_html(&link);

    let name = extract_name(&html);
    let ingredients = extract_ingredients(&html);
    let preparation = extract_preparation(&html);

    Recipe::new(name, ingredients, preparation)
}
