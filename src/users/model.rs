use crate::schema::users;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: String
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub username: String,
    pub password: String,
    pub name: String
}

impl InsertableUser {
    fn from_user(user: User) -> InsertableUser {
        InsertableUser {
            username: user.username,
            password: user.password,
            name: user.name,
        }
    }
}

impl User {
    pub fn create(user: User, connection: &PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
        .values(&InsertableUser::from_user(user))
        .execute(connection)?;

        users::table.order(users::id.desc()).first(connection)
    }
}
