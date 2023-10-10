use askama::Template;

#[derive(Template)]
#[template(path = "components/button.html")]
pub struct Button;
