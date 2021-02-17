pub mod model;

use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};
use self::model::Post;

use serde::{Serialize,Deserialize};

use crate::users::model::User;
use crate::users::auth::ApiKey;
use crate::users::auth::jwt::{
    Header,
    Registered,
    Token,
};


use crate::db;

#[post("/", data="<post>")]
fn create(_key: ApiKey, post: Json<Post>, connection: db::DbConn) -> Result<Json<Post>, Status> {
    Post::create(post.into_inner(), &connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/", data="<post>", rank=2)]
fn create_error(post: Json<Post>) -> Json<JsonValue> {
    Json(json!(
            {
                "success":false,
                "message": "Not authorized"
            }
        ))
}

#[get("/")]
fn read(connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    Post::get_all(&connection)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/posts", routes![create,read,create_error])
}
