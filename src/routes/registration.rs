use std::{collections::HashMap};
use rocket::response::content::RawHtml;
use crate::utilities::page::Render;

#[get("/registration")]
pub async fn registration() -> RawHtml<String> {
    let mut environment = vec![];

    let mut page_template = include_str!("../../views/registration.template.html").to_string();
    let page = page_template.render(&environment).unwrap();

    environment.push(("children".to_string(), Box::new(page)));

    let mut layout_template = include_str!("../../views/layout.template.html").to_string();
    let mut layout = layout_template.render(&environment).unwrap();

    RawHtml(layout.render(&environment).unwrap())
}
