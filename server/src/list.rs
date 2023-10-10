use std::fmt::Display;

use askama::Template;
use chrono::{DateTime, Utc};

#[derive(Template)]
#[template(path = "list.html")]
pub struct List {
    pub list_items: Vec<WeightEntry>,
    pub err: Option<String>,
}

#[derive(Clone, Debug)]
pub struct WeightEntry {
    pub id: i16,
    pub weight_type: String,
    pub mass: i32,
    pub reps: i32,
    pub date: DateTime<Utc>,
}

impl Display for WeightEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "weight_type: {}, mass: {}, reps: {}, date: {}",
            self.weight_type, self.mass, self.reps, self.date
        )
    }
}
