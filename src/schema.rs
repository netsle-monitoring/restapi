table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        hashed_pw -> Binary,
        refresh_token -> Text,
        salt -> Binary,
    }
}
