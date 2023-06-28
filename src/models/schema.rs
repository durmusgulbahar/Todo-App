// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        done -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        #[max_length = 255]
        user_id -> Varchar,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
