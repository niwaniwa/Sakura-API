// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int8,
        #[max_length = 255]
        username -> Varchar,
        card_id -> Bytea,
    }
}
