use std::collections::HashMap;

pub struct Filter {
    criteria: HashMap<String, String>,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            criteria: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.criteria.insert(key.to_string(), value.to_string());
    }

    pub fn matches(&self, hotel: &super::models::Hotel) -> bool {
      for (key, value) in &self.criteria {
          let value = value.to_lowercase();
          match key.as_str() {
              "holiday_reference" => if !hotel.holiday_reference.to_string().to_lowercase().contains(&value) { return false; },
              "hotel_name" => if !hotel.hotel_name.to_lowercase().contains(&value) { return false; },
              "city" => if !hotel.city.to_lowercase().contains(&value) { return false; },
              "continent" => if !hotel.continent.to_lowercase().contains(&value) { return false; },
              "country" => if !hotel.country.to_lowercase().contains(&value) { return false; },
              "category" => if !hotel.category.to_lowercase().contains(&value) { return false; },
              "star_rating" => if !hotel.star_rating.to_string().to_lowercase().contains(&value) { return false; },
              "temp_rating" => if !hotel.temp_rating.to_lowercase().contains(&value) { return false; },
              "location" => if !hotel.location.to_lowercase().contains(&value) { return false; },
              "price_per_night" => if !hotel.price_per_night.to_string().to_lowercase().contains(&value) { return false; },
              _ => {},
          }
      }
  
      true
  }  
}