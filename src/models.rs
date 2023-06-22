use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hotel {
    pub holiday_reference: u32,
    pub hotel_name: String,
    pub city: String,
    pub continent: String,
    pub country: String,
    pub category: String,
    pub star_rating: u32,
    pub temp_rating: String,
    pub location: String,
    pub price_per_night: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub from: String,
    pub timestamp: String,
}
