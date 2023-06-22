// message.rs

use rocket::State;
use rocket_contrib::json::Json;
use rocket::response::content;
use serde::Deserialize;
use crate::models::AppState;
use crate::filter::Filter;

#[derive(Deserialize)]
pub struct Message {
    message: String,
}

#[post("/message", format = "json", data = "<message>")]
pub fn message(message: Json<Message>, state: State<AppState>) -> content::Plain<String> {
    let message = message.message.to_lowercase();
    let words: Vec<&str> = message.split_whitespace().collect();

    let mut filter = Filter::new();

    for word in &words {
        filter.add("holiday_reference", word);
        filter.add("hotel_name", word);
        filter.add("city", word);
        filter.add("continent", word);
        filter.add("country", word);
        filter.add("category", word);
        filter.add("star_rating", word);
        filter.add("temp_rating", word);
        filter.add("location", word);
        filter.add("price_per_night", word);
    }

    let hotels: Vec<&Hotel> = state.hotels.iter().filter(|hotel| filter.matches(hotel)).collect();

    if hotels.is_empty() {
        content::Plain("No hotel found with that criteria".to_string())
    } else {
        content::Plain(format!("Found hotels: {:?}", hotels))
    }
}
