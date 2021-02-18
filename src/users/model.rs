use crate::schema::users;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde::{Serialize,Deserialize};

use bcrypt::{DEFAULT_COST, hash, verify};

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
        let encrypted_user = User {
            password: hash(user.password,DEFAULT_COST).unwrap(),
            ..user
        };
        diesel::insert_into(users::table)
        .values(&InsertableUser::from_user(encrypted_user))
        .execute(connection)?;

        users::table.order(users::id.desc()).first(connection)
    }

    pub fn get_by_username_and_password(username_: String, password_: String, connection: &PgConnection) -> Option<User> {
        let res = users::table
            .filter(users::username.eq(username_))
            .get_result::<User>(connection);
        match res {
            Ok(user) => {
                if let Ok(matching) = verify(&password_,&user.password) {
                    if matching {
                        return Some(user)
                    }
                }
                return None
            }
            Err(_) => {
                None
            }
        }
    }
}
