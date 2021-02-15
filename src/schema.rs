table! {
    blacklist (id) {
        id -> Nullable<Integer>,
        ip -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        hashed_pw -> Binary,
        refresh_token -> Text,
        salt -> Binary,
        is_admin -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    blacklist,
    users,
);
