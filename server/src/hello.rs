use askama::Template;

use crate::add::Add;
use crate::list::List;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloWorld {
    pub title: &'static str,
    pub list: List,
    pub add: Add,
}
