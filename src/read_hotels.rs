use std::error::Error;
use csv::Reader;

use crate::models::Hotel;

pub fn read_hotels() -> Result<Vec<Hotel>, Box<dyn Error>> {
  let mut reader = Reader::from_path("hotels.csv")?;
  let mut hotels = Vec::new();

  for result in reader.deserialize() {
      let hotel: Hotel = result?;
      hotels.push(hotel);
  }

  Ok(hotels)
}
