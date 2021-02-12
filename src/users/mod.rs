pub mod model;

use rocket::{self, http::Status};
use rocket_contrib::json::Json;
use self::model::User;

use crate::db;

#[post("/", format="application/json", data = "<user>")]
fn create(user: Json<User>, connection: db::DbConn) -> Result<Json<User>, Status> {
    User::create(user.into_inner(), &connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}


pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/user", routes![create])
}