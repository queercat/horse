use std::{collections::HashMap};
use rocket::{State, response::content::RawHtml};
use crate::{services::post_service::PostService, utilities::page::Render};

#[get("/")]
pub async fn index(post_service: &State<PostService>) -> RawHtml<String> {
    let mut environment = HashMap::<String, String>::new();

    let mut page_template = include_str!("../../views/index.template.html").to_string();
    let page = page_template.render(Some(&environment)).unwrap();

    let posts = post_service.get_posts().await;

    environment.insert("children".to_string(), page);

    let mut layout_template = include_str!("../../views/layout.template.html").to_string();
    let mut layout = layout_template.render(Some(&environment)).unwrap();

    RawHtml(layout.render(Some(&environment)).unwrap())
}
