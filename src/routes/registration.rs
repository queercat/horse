use rocket::response::content::RawHtml;

#[get("/registration")]
pub async fn registration() -> RawHtml<String> {
    todo!()
}
