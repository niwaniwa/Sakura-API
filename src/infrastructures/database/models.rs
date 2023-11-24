use super::super::database::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, Insertable)]
#[table_name = "account"]
pub struct NewAccountEntity {
    pub username: String,
    pub card_id: Vec<u8>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, AsChangeset)]
#[table_name = "account"]
pub struct AccountEntity {
    pub id: i64,
    pub username: String,
    pub card_id: Vec<u8>,
    pub created_at: NaiveDateTime,
}
