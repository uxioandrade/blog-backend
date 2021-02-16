pub mod model;
pub mod auth;

use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};
use self::model::User;
use self::auth::ApiKey;
use self::auth::crypto::sha2::Sha256;
use self::auth::jwt::{
    Header,
    Registered,
    Token,
};

use serde::{Serialize,Deserialize};

use crate::db;

#[post("/", format="application/json", data = "<user>")]
fn create(user: Json<User>, connection: db::DbConn) -> Result<Json<User>, Status> {
    User::create(user.into_inner(), &connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/info")]
fn info(key: ApiKey) -> Json<JsonValue> {
    Json(json!(
        {
            "success": true,
            "message": key.0
        }
    ))
}

#[derive(Serialize, Deserialize)]
struct Credentials {
   username: String,
   password: String
}

#[post("/login", data = "<credentials>")]
fn login(credentials: Json<Credentials>, connection: db::DbConn) ->  Result<Json<JsonValue>, Status> {
let header: Header = Default::default();
    let username = credentials.username.to_string();
    let password = credentials.password.to_string();
    
    match User::get_by_username_and_password(username, password, &connection) {
        None => {
            Err(Status::NotFound)
        },
        Some(user) => {
            let claims = Registered {
                sub: Some(user.name.into()),
                ..Default::default()
            };
            let token = Token::new(header, claims);

            token.signed(b"secret_key", Sha256::new())
                .map(|message| Json(json!({ "success": true, "token": message })))
                .map_err(|_| Status::InternalServerError)
        }
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/user", routes![create,info])
        .mount("/auth", routes![login])
}