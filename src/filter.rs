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
            match key.as_str() {
                "holiday_reference" => if !hotel.holiday_reference.to_string().contains(value) { return false; },
                "hotel_name" => if !hotel.hotel_name.contains(value) { return false; },
                "city" => if !hotel.city.contains(value) { return false; },
                "continent" => if !hotel.continent.contains(value) { return false; },
                "country" => if !hotel.country.contains(value) { return false; },
                "category" => if !hotel.category.contains(value) { return false; },
                "star_rating" => if !hotel.star_rating.to_string().contains(value) { return false; },
                "temp_rating" => if !hotel.temp_rating.contains(value) { return false; },
                "location" => if !hotel.location.contains(value) { return false; },
                "price_per_night" => if !hotel.price_per_night.to_string().contains(value) { return false; },
                _ => {},
            }
        }

        true
    }
}