use rocket::response::content::RawHtml;

#[get("/home")]
pub async fn home() -> RawHtml<String> {
    todo!()
}
