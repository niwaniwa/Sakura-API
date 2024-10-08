use chrono::NaiveDateTime;

use super::super::database::schema::*;

#[derive(Debug, Insertable)]
#[table_name = "account"]
pub struct NewAccountEntity {
    pub username: String,
    pub grade: i32,
    pub expiration_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub auth_id: i64,
}

#[derive(Debug, Queryable, Identifiable, AsChangeset)]
#[table_name = "account"]
pub struct AccountEntity {
    pub id: i64,
    pub username: String,
    pub grade: i32,
    pub expiration_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub auth_id: i64,
}

#[derive(Debug, Insertable)]
#[table_name = "card"]
pub struct NewCardEntity {
    pub account_id: i64,
    pub card_name: String,
    pub card_number: Vec<u8>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, AsChangeset)]
#[table_name = "card"]
pub struct CardEntity {
    pub id: i64,
    pub account_id: i64,
    pub card_name: String,
    pub card_number: Vec<u8>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "door"]
pub struct NewDoorEntity {
    pub device_id: String,
    pub door_state: bool,
    pub door_switch_state: bool,
}

#[derive(Debug, Queryable, AsChangeset, Insertable)]
#[table_name = "door"]
pub struct DoorEntity {
    pub device_id: String,
    pub door_state: bool,
    pub door_switch_state: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "auth"]
pub struct NewAuthEntity {
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, AsChangeset)]
#[table_name = "auth"]
pub struct AuthEntity {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}
