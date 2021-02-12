#![feature(decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::response::content::Json;


mod db;
mod users;
pub mod schema;

#[get("/ping")]
fn ping() -> Json<&'static str>{
    Json("{
        'status': 'sucess',
        'message': 'pong'
    }")
}

fn main() {
    let mut rocket = rocket::ignite()
        .manage(db::init_pool());
    rocket = users::mount(rocket);
    rocket.launch();
}
