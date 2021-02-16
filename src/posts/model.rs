use crate::schema::posts;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name="posts"]
pub struct Post {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub image_url: String,
    pub date: String,
    pub length: Option<i16>
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct InsertablePost {
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub image_url: String,
    pub date: String,
    pub length: Option<i16>
}

impl InsertablePost {
    fn from_post(post: Post) -> InsertablePost {
        InsertablePost {
            url: post.url,
            title: post.title,
            description: post.description,
            image_url: post.image_url,
            date: post.date,
            length: post.length
        }
    }
}

impl Post {
    pub fn create(post: Post, connection: &PgConnection) -> QueryResult<Post> {
        diesel::insert_into(posts::table)
            .values(&InsertablePost::from_post(post))
            .execute(connection)?;
        posts::table.order(posts::id.desc()).first(connection)
    }

    pub fn get_all(connection: &PgConnection) -> QueryResult<Vec<Post>> {
        posts::table.order(posts::id).load::<Post>(connection)
    }
}