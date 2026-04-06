// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Text,
        description -> Nullable<Text>,
        content -> Text,
        author -> Text,
        created_at -> Timestamp,
        last_updated -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        nickname -> Text,
        password_hash -> Text,
        about -> Nullable<Text>,
        created_at -> Timestamp,
        last_updated -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(posts, users,);
