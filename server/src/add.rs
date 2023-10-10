use askama::Template;

#[derive(Template)]
#[template(path = "add.html")]
pub struct Add {
    pub weight_types: Vec<String>,
}
