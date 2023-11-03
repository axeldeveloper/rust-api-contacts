// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Int4,
        name -> Varchar,
        cpf -> Varchar,
        //age -> Int4,
        //published -> Bool,
    }
}

diesel::table! {
    likes (id) {
        id -> Uuid,
        created_at -> Timestamp,
        tweet_id -> Uuid,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    tweets (id) {
        id -> Uuid,
        created_at -> Timestamp,
        message -> Text,
    }
}

diesel::joinable!(likes -> tweets (tweet_id));

diesel::allow_tables_to_appear_in_same_query!(
    contacts,
    likes,
    posts,
    tweets,
);
