use askama::Template;

use crate::components::Button;

#[derive(Template)]
#[template(path = "add.html")]
pub struct Add {
    pub weight_types: Vec<String>,
    pub button: Button,
}
