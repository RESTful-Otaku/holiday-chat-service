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
                "city" => if hotel.city != *value { return false; },
                "star_rating" => if hotel.star_rating.to_string() != *value { return false; },
                "temp_rating" => if hotel.temp_rating != *value { return false; },
                _ => {},
            }
        }

        true
    }
}
