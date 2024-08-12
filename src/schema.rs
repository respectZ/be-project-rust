// @generated automatically by Diesel CLI.

diesel::table! {
    company (id) {
        id -> Int8,
        created_at -> Timestamptz,
        name -> Varchar,
    }
}

diesel::table! {
    company_position (id) {
        id -> Int8,
        created_at -> Timestamptz,
        position_id -> Int8,
        company_id -> Int8,
    }
}

diesel::table! {
    follows (id) {
        id -> Int8,
        created_at -> Timestamptz,
        following_user_id -> Int8,
        followed_user_id -> Int8,
    }
}

diesel::table! {
    position (id) {
        id -> Int8,
        created_at -> Timestamptz,
        name -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int8,
        created_at -> Timestamptz,
        body -> Text,
        user_id -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        created_at -> Timestamptz,
        email -> Varchar,
        username -> Varchar,
        profile_picture -> Nullable<Varchar>,
        password -> Varchar,
        company_position_id -> Nullable<Int8>,
        role -> Int8,
    }
}

diesel::joinable!(company_position -> company (company_id));
diesel::joinable!(company_position -> position (position_id));
diesel::joinable!(posts -> users (user_id));
diesel::joinable!(users -> company_position (company_position_id));

diesel::allow_tables_to_appear_in_same_query!(
    company,
    company_position,
    follows,
    position,
    posts,
    users,
);
