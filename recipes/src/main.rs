use std::error::Error;

use now_lambda::{error::NowError, lambda, Request, Response};
use prost::Message;

use recipes::get_recipe;

fn handler(_: Request) -> Result<Response<Vec<u8>>, NowError> {
    let recipe = get_recipe();
    let mut b: Vec<u8> = Vec::new();
    recipe.encode(&mut b).unwrap();

    let response = Response::builder().body(b).expect("Failed to render response");

    Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}