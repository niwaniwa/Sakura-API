pub struct Account {
    pub id: u64,
    pub username: String,
    pub card_id: u64,
}

impl Account {
    pub fn create(username: String, card_id: u64) -> Self {
        Self {
            id: Default::default(),
            username,
            card_id,
        }
    }
    pub fn register_card_id() -> u8 {
        //TODO
        let card_id = 120;
        return card_id;
    }
}
