table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        hashed_pw -> Text,
        refresh_token -> Text,
        salt -> Text,
    }
}
