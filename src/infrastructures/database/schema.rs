// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int8,
        #[max_length = 255]
        username -> Varchar,
        grade -> Int4,
        expiration_date -> Timestamptz,
        created_at -> Timestamptz,
        auth_id -> Int8,
    }
}

diesel::table! {
    auth (id) {
        id -> Int8,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    card (id) {
        id -> Int8,
        account_id -> Int8,
        #[max_length = 255]
        card_name -> Varchar,
        card_number -> Bytea,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    door (device_id) {
        #[max_length = 255]
        device_id -> Varchar,
        door_state -> Bool,
        door_switch_state -> Bool,
    }
}

diesel::joinable!(account -> auth (auth_id));
diesel::joinable!(card -> account (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    auth,
    card,
    door,);
