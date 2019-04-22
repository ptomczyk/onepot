use url::ParseError;

use prost::Message;
use rand::seq::SliceRandom;
use rand::Rng;
use reqwest::{header, Client};
use scraper::{Html, Selector};
use url::Url;


use crate::items::{Error as RecipeError, Ingredient, Recipe, Step};

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/onepot.rs"));
}

impl RecipeError {
    fn new(reason: FetchError) -> Self {
        let mut err = Self::default();

        err.reason = match reason {
            FetchError::UrlParsingError => String::from("URL parsing error"),
            FetchError::ParseError => String::from("HTML parsing error"),
            FetchError::UnknownError => String::from("Other error"),
        };

        err
    }
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

enum FetchError {
    UrlParsingError,
    ParseError,
    UnknownError,
}

impl From<ParseError> for FetchError {
    fn from(_: ParseError) -> Self {
        FetchError::UrlParsingError
    }
}

fn fetch_html(url: &str) -> Result<Html, FetchError> {
    let url = Url::parse(&url)?;

    let client = Client::new();
    let mut res = client
        .get(url)
        .header(header::ACCEPT, "text/html")
        .send()
        .expect("Failed to send HTTP request");

    let html = match res.text() {
        Ok(html) => html,
        Err(_) => return Err(FetchError::ParseError),
    };
    let html = Html::parse_document(&html);

    Ok(html)
}

fn get_url(endpoint: &str) -> String {
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

fn extract_links() -> Result<Vec<String>, FetchError> {
    let mut rng = rand::thread_rng();
    let page = rng.gen_range(0, 4);

    let url = &get_url("/przepisy/jednogarnkowe");
    let url = format!("{}?page={}", url, page);

    let html = fetch_html(&url)?;
    let links = extract_hrefs(&html);

    Ok(links)
}

fn get_random_link() -> Result<String, FetchError> {
    let links = extract_links()?;
    let link = links.choose(&mut rand::thread_rng());
    match link {
        Some(link) => Ok(link.to_owned()),
        None => Err(FetchError::UnknownError),
    }
}

pub fn get_recipe(buffer: &mut Vec<u8>) -> () {
    let link = match get_random_link() {
        Ok(link) => link,
        Err(err) => {
            RecipeError::new(err).encode(buffer).unwrap();
            return ();
        }
    };

    let link = get_url(&link);
    let html = match fetch_html(&link) {
        Ok(html) => html,
        Err(err) => {
            RecipeError::new(err).encode(buffer).unwrap();
            return ();
        }
    };

    let name = extract_name(&html);
    let ingredients = extract_ingredients(&html);
    let preparation = extract_preparation(&html);

    let recipe = Recipe::new(name, ingredients, preparation);
    recipe.encode(buffer).unwrap();
}
