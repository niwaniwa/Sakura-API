use super::super::database::schema::*;

#[derive(Debug, Insertable)]
#[table_name = "account"]
pub struct NewAccountEntity {
    pub username: String,
    pub card_id: Vec<u8>,
}
