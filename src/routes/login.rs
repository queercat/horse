use rocket::response::content::RawHtml;

#[get("/login")]
pub async fn login() -> RawHtml<String> {
    todo!()
}
