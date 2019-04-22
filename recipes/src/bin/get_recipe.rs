use std::error::Error;

use now_lambda::{lambda, Request, Response};

use recipes::get_recipe;

fn handler(_: Request) -> Result<Response<Vec<u8>>, http::Error> {
    let mut b: Vec<u8> = Vec::new();
    get_recipe(&mut b);

    let response = Response::builder().body(b);

    response
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}