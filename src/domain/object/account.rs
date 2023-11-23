pub struct Account {
    pub id: u64,
    pub username: String,
    pub card_id: Vec<u8>,
}

impl Account {
    pub fn create(username: String, card_id: Vec<u8>) -> Self {
        Self {
            id: Default::default(),
            username,
            card_id,
        }
    }
    pub fn register_card_id() -> Vec<u8> {
        //TODO
        let card_id: Vec<u8> = vec![1, 16, 3, 16, 197, 20, 106, 38];
        return card_id;
    }
}
