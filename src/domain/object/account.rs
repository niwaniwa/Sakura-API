pub struct Account {
    pub id: u64,
    pub username: String,
    pub card_id: u8,
}

impl Account {
    pub fn create(username: String) -> Self {
        Self {
            id: Default::default(),
            username,
            card_id: Default::default(),
        }
    }
}
