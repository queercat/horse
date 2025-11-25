use crate::utilities::page::TemplateEngine;
use rocket::{response::content::RawHtml};

#[get("/layout")]
pub async fn layout() -> RawHtml<String> {
    let mut template_engine = TemplateEngine::default();
    RawHtml(template_engine.render("layout").unwrap())
}
