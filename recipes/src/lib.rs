use url::ParseError;

use prost::Message;
use rand::seq::SliceRandom;
use rand::Rng;
use reqwest::{header, Client};
use scraper::{Html};
use url::Url;

use crate::items::{Error as RecipeError, Ingredient, Recipe, Step};

mod extractors;
use extractors::{extract_hrefs, extract_ingredients, extract_name, extract_preparation};

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/onepot.rs"));
}

impl RecipeError {
    fn new(reason: FetchError) -> Self {
        let mut err = Self::default();

        err.reason = match reason {
            FetchError::UrlParsingError => String::from("URL parsing error"),
            FetchError::FetchHtmlError(e) => String::from("Reqwest error"),
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
    FetchHtmlError(reqwest::Error),
    UnknownError,
}

impl From<ParseError> for FetchError {
    fn from(_: ParseError) -> Self {
        FetchError::UrlParsingError
    }
}

fn fetch_html(url: Url) -> Result<Html, FetchError> {
    let client = Client::new();
    let mut res = client
        .get(url)
        .header(header::ACCEPT, "text/html")
        .send()
        .expect("Failed to send HTTP request");

    let html = match res.text() {
        Ok(html) => html,
        Err(err) => return Err(FetchError::FetchHtmlError(err)),
    };
    let html = Html::parse_document(&html);

    Ok(html)
}

fn get_url(endpoint: &str) -> Result<Url, ParseError> {
    let url = format!("https://www.kwestiasmaku.com{}", endpoint);
    let url = Url::parse(&url)?;
    Ok(url)
}


fn extract_links() -> Result<Vec<String>, FetchError> {
    let mut rng = rand::thread_rng();
    let page = rng.gen_range(0, 4);

    let url = get_url(&format!("/przepisy/jednogarnkowe?page={}", page))?;

    let html = fetch_html(url)?;
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

    let url = get_url(&link).ok().unwrap();
    let html = match fetch_html(url) {
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
