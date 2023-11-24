use super::Id;
use chrono::{Local, NaiveDateTime};

pub type AccountId = Id<Account>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    pub username: String,
    pub card_id: Vec<u8>,
    pub created_at: NaiveDateTime,
}

impl Account {
    pub fn create(username: String, card_id: Vec<u8>) -> Self {
        Self {
            id: Default::default(),
            username,
            card_id,
            created_at: create_time(),
        }
    }
    pub fn register_card_id() -> Vec<u8> {
        //TODO
        let card_id: Vec<u8> = vec![1, 16, 3, 16, 197, 20, 106, 38];
        card_id
    }
}

fn create_time() -> NaiveDateTime {
    let local_now = Local::now();
    local_now.naive_local()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let username = "test_user".to_string();
        let card_id = [1, 16, 3, 16, 197, 20, 106, 38].to_vec();

        let account = Account::create(username.clone(), card_id.clone());

        assert_eq!(account.id.get(), 0);
        assert_eq!(account.username, username);
        assert_eq!(account.card_id, card_id);
    }
}
