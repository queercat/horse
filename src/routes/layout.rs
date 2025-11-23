use rocket::response::content::RawHtml;

#[get("/layout")]
pub async fn layout() -> RawHtml<String> {
    todo!()
}
