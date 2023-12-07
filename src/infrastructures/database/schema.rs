// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int8,
        #[max_length = 255]
        username -> Varchar,
        grade -> Int4,
        #[max_length = 255]
        card_type -> Varchar,
        card_id -> Bytea,
        created_at -> Timestamptz,
    }
}
