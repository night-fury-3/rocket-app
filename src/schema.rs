// @generated automatically by Diesel CLI.

diesel::table! {
    rustaceans (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}
