// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        user_id -> Varchar,
        address -> Varchar,
        keystore -> Json,
    }
}
