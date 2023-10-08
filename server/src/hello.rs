use askama::Template;

use crate::list::Row;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloWorld<'a> {
    pub title: &'a str,
    pub name: &'a str,
    pub list_items: Vec<Row>,
}
