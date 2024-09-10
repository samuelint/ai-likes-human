// @generated automatically by Diesel CLI.

diesel::table! {
    configuration (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
    }
}
