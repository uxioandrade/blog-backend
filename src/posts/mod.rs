pub mod model;

use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};
use self::model::Post;

use serde::{Serialize,Deserialize};

use crate::db;

#[post("/", data="<post>")]
fn create(post: Json<Post>, connection: db::DbConn) -> Result<Json<Post>, Status> {
    Post::create(post.into_inner(), &connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/")]
fn read(connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    Post::get_all(&connection)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/posts", routes![create,read])
}