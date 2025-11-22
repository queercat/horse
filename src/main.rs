#[macro_use]
extern crate rocket;

mod database;
mod models;

use std::error::Error;

use dotenv;
use rocket::State;
use rocket::{fs::FileServer, response::content::RawHtml};
use rshtml::traits::RsHtml;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Statement};

use crate::database::setup::set_up_db;
use crate::models::pages::{HomePage, IndexPage};

#[get("/")]
async fn index(db: &State<DatabaseConnection>) -> RawHtml<String> {
    let post = db
        .query_one_raw(Statement::from_string(
            db.get_database_backend(),
            "SELECT * FROM user where id = 0;",
        ))
        .await
        .unwrap()
        .unwrap();

    let username: String = post.try_get("", "username").unwrap();

    let mut page = HomePage {
        title: username
    };

    RawHtml(page.render().unwrap())
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let db = match set_up_db().await {
        Ok(db) => db,
        _ => panic!("Unable to initialize database"),
    };

    rocket::build()
        .manage(db)
        .mount("/", routes![index])
        .mount("/", FileServer::from("./public"))
}
