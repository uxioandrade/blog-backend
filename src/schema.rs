table! {
    posts (id) {
        id -> Int4,
        url -> Varchar,
        title -> Varchar,
        description -> Nullable<Varchar>,
        image_url -> Varchar,
        date -> Varchar,
        length -> Nullable<Int2>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
