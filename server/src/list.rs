use std::fmt::Display;

use askama::Template;

#[derive(Template)]
#[template(path = "list.html")]
pub struct List {
    pub list_items: Vec<Row>,
}

#[derive(Clone, Debug)]
pub struct Row {
    pub id: i16,
    pub name: String,
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, name: {}", self.id, self.name)
    }
}
