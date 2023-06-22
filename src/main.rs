#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::State;
use rocket_cors::{AllowedOrigins, CorsOptions};

mod models;
mod filter;
mod message;
mod read_hotels;

use models::Hotel;
use filter::Filter;
use message::message;
use read_hotels::read_hotels;

struct AppState {
  hotels: Vec<Hotel>,
}

fn main() {
  let hotels = read_hotels().unwrap();

  let cors = CorsOptions::default()
      .allowed_origins(AllowedOrigins::all())
      .allow_credentials(true)
      .to_cors().unwrap();

  rocket::ignite()
      .mount("/", routes![message])
      .attach(cors)
      .manage(AppState { hotels })
      .launch();
}
