#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
        card_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let username = "test_user".to_string();
        let card_id = [1, 16, 3, 16, 197, 20, 106, 38].to_vec();

        let account = Account::create(username.clone(), card_id.clone());

        assert_eq!(account.id, 0);
        assert_eq!(account.username, username);
        assert_eq!(account.card_id, card_id);
    }
}
