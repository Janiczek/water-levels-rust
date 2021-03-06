#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use water_levels::rain;
use std::env;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use rocket::Config;
use rocket::config::Environment;

#[derive(Deserialize)]
struct Input {
    landscape: Vec<i32>,
    hours: i32,
}

#[derive(Serialize)]
struct Output {
    result: Vec<f32>,
}

#[post("/rain", format = "json", data = "<input>")]
fn post_rain(input: Json<Input>) -> JsonValue {
    let result = rain(&input.0.landscape, input.0.hours);
    json!(Output { result })
}

#[allow(clippy::float_cmp)]
fn main() {
    assert_eq!(rain(&[3, 1, 6, 4, 5, 6], 1), [3.5, 3.5, 6., 6., 6., 6.]);
    assert_eq!(rain(&[3, 1, 6, 4, 8, 9], 1), [4., 4., 6., 6., 8., 9.]);
    assert_eq!(rain(&[1, 9, 1], 1), [2.5, 9., 2.5]);
    assert_eq!(rain(&[1, 1, 9], 1), [2.5, 2.5, 9.]);
    assert_eq!(rain(&[8, 8, 1], 1), [8., 8., 4.]);
    assert_eq!(rain(&[8, 8, 8, 1], 1), [8., 8., 8., 5.]);
    assert_eq!(rain(&[8, 1, 8, 8, 1], 1), [8., 4., 8., 8., 3.]);
    assert_eq!(rain(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 1), [4.75, 4.75, 4.75, 4.75, 5., 6., 7., 8., 9.]);

    let port: u16 = env::var("PORT")
        .map(|str| str.parse::<u16>().unwrap())
        .unwrap_or(8000);

    let config = Config::build(Environment::Staging)
        .port(port)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![post_rain])
        .launch();
}
