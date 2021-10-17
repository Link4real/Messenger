table! {
    messages (id) {
        id -> Int4,
        author -> Varchar,
        body -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        pwhash -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
