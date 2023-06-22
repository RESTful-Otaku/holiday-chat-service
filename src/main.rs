#[macro_use] extern crate rocket;

use rocket::State;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};

mod models;
mod filter;

use models::Hotel;
use filter::Filter;

struct AppState {
    hotels: Vec<Hotel>,
}

#[derive(Deserialize)]
struct Message {
    message: String,
}

#[post("/message", format = "json", data = "<message>")]
fn message(message: Json<Message>, state: State<AppState>) -> content::Plain<String> {
    // TODO: message handling logic goes here 
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true)
        .to_cors().unwrap();

    rocket::build()
        .mount("/", routes![message])
        .attach(cors)
        .manage(AppState { hotels: read_hotels().unwrap() })
}
