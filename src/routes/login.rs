use crate::utilities::page::TemplateEngine;
use rocket::{response::content::RawHtml};

#[get("/login")]
pub async fn login() -> RawHtml<String> {
    let mut template_engine = TemplateEngine::default();
    RawHtml(template_engine.render("login").unwrap())
}
