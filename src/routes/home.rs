use crate::utilities::page::TemplateEngine;
use rocket::{response::content::RawHtml};

#[get("/home")]
pub async fn home() -> RawHtml<String> {
    let mut template_engine = TemplateEngine::default();
    RawHtml(template_engine.render("home").unwrap())
}
