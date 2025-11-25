use crate::utilities::page::TemplateEngine;
use rocket::{response::content::RawHtml};

#[get("/register")]
pub async fn register() -> RawHtml<String> {
    let mut template_engine = TemplateEngine::default();
    RawHtml(template_engine.render("register").unwrap())
}
