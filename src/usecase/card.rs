use crate::domain::object::{
    account::AccountId,
    card::{Card, CardId},
};
use crate::domain::repository::{account::AccountRepository, card::CardRepository};
use anyhow;

pub fn post_card(
    card_repository: &impl CardRepository,
    account_repository: &impl AccountRepository,
    card: &Card,
) -> anyhow::Result<()> {
    let account_id = &card.account_id;
    match account_repository.find_by_id(account_id) {
        Ok(_) => {
            card_repository.insert(card)?;
            Ok(())
        }
        Err(err) => Err(anyhow::anyhow!("Failed to find account: {}", err)),
    }
}

pub fn get_card_list(
    card_repository: &impl CardRepository,
    account_repository: &impl AccountRepository,
    account_id: &AccountId,
) -> anyhow::Result<Vec<Card>> {
    match account_repository.find_by_id(account_id) {
        Ok(_) => {
            let cards = card_repository.list(account_id)?;
            Ok(cards)
        }
        Err(err) => Err(anyhow::anyhow!("Failed to find account: {}", err)),
    }
}

pub fn get_card(
    card_repository: &impl CardRepository,
    account_repository: &impl AccountRepository,
    card_id: &CardId,
    account_id: &AccountId,
) -> anyhow::Result<Card> {
    match account_repository.find_by_id(account_id) {
        Ok(_) => {
            let card = card_repository.find_by_id(card_id, account_id)?;
            Ok(card)
        }
        Err(err) => Err(anyhow::anyhow!("Failed to find account: {}", err)),
    }
}

pub fn delete_card(
    card_repository: &impl CardRepository,
    account_repository: &impl AccountRepository,
    card_id: &CardId,
    account_id: &AccountId,
) -> anyhow::Result<()> {
    match account_repository.find_by_id(account_id) {
        Ok(_) => {
            let card = card_repository.find_by_id(card_id, account_id)?;
            card_repository.delete(&card)?;
            Ok(())
        }
        Err(err) => Err(anyhow::anyhow!("Failed to find account: {}", err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::object::{
        account::{Account, AccountId},
        auth::AuthId,
        card::{Card, CardId},
    };
    use crate::tests::{
        mock_account_repository::MockAccountRepository, mock_card_repository::MockCardRepository,
    };
    use crate::usecase::account::*;
    use chrono::{Duration, Local};
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[test]
    fn success_post_card() {
        let account_repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let card_repository = MockCardRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            auth_id: AuthId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let test_card = Card {
            id: CardId::new(1),
            account_id: AccountId::new(1),
            card_name: "suica".to_string(),
            card_number: [1, 16, 3, 16, 197, 20, 106, 38].to_vec(),
            created_at: Local::now().naive_local(),
        };

        let _ = post_account(&account_repository, &test_account);

        let result = post_card(&card_repository, &account_repository, &test_card);

        assert!(result.is_ok());
    }

    #[test]
    fn success_get_cards() {
        let account_repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let card_repository = MockCardRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            auth_id: AuthId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let test_card = Card {
            id: CardId::new(1),
            account_id: AccountId::new(1),
            card_name: "suica".to_string(),
            card_number: [1, 16, 3, 16, 197, 20, 106, 38].to_vec(),
            created_at: Local::now().naive_local(),
        };

        let test_card2 = Card {
            id: CardId::new(2),
            account_id: AccountId::new(1),
            card_name: "suica".to_string(),
            card_number: [1, 16, 3, 17, 200, 20, 100, 20].to_vec(),
            created_at: Local::now().naive_local(),
        };

        let _ = account_repository.insert(&test_account);

        let _ = card_repository.insert(&test_card);
        let _ = card_repository.insert(&test_card2);

        let result = get_card_list(&card_repository, &account_repository, &test_card.account_id);

        let cards = result.unwrap();
        assert_eq!(cards.len(), 2);
    }

    #[test]
    fn success_get_card() {
        let account_repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let card_repository = MockCardRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            auth_id: AuthId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let test_card = Card {
            id: CardId::new(1),
            account_id: AccountId::new(1),
            card_name: "suica".to_string(),
            card_number: [1, 16, 3, 16, 197, 20, 106, 38].to_vec(),
            created_at: Local::now().naive_local(),
        };
        let _ = account_repository.insert(&test_account);
        let _ = card_repository.insert(&test_card);

        let result = get_card(
            &card_repository,
            &account_repository,
            &test_card.id,
            &test_account.id,
        );

        assert!(result.is_ok());

        let retrieved_card = result.unwrap();

        assert_eq!(retrieved_card.id.get(), test_card.id.get());
        assert_eq!(retrieved_card.account_id, test_card.account_id);
        assert_eq!(retrieved_card.card_name, test_card.card_name);
        assert_eq!(retrieved_card.card_number, test_card.card_number);
        assert_eq!(retrieved_card.created_at, test_card.created_at);
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn find_by_card_number_existing_card() {
        let account_repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let card_repository = MockCardRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            auth_id: AuthId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let test_card = Card {
            id: CardId::new(1),
            account_id: AccountId::new(1),
            card_name: "suica".to_string(),
            card_number: [1, 16, 3, 16, 197, 20, 106, 38].to_vec(),
            created_at: Local::now().naive_local(),
        };
        let _ = account_repository.insert(&test_account);
        let _ = card_repository.insert(&test_card);

        let result = card_repository.find_by_card_number(&test_card.card_number);
        assert!(result.is_ok(), "Result was an error: {:?}", result);
        assert_eq!(result.unwrap(), true)
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn find_by_card_number_non_existing_card() {
        let account_repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let card_repository: MockCardRepository = MockCardRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            auth_id: AuthId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let test_card = Card {
            id: CardId::new(1),
            account_id: AccountId::new(1),
            card_name: "suica".to_string(),
            card_number: [1, 16, 3, 16, 197, 20, 106, 38].to_vec(),
            created_at: Local::now().naive_local(),
        };

        let dummy_card = Card {
            id: CardId::new(2),
            account_id: AccountId::new(1),
            card_name: "suica".to_string(),
            card_number: [1, 10, 3, 55, 20, 106, 3].to_vec(),
            created_at: Local::now().naive_local(),
        };
        let _ = account_repository.insert(&test_account);
        let _ = card_repository.insert(&test_card);

        let result = card_repository.find_by_card_number(&dummy_card.card_number);
        assert!(result.is_ok(), "Result was an error: {:?}", result);
        assert_eq!(result.unwrap(), false)
    }

    #[test]
    fn success_delete_card() {
        let account_repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let card_repository: MockCardRepository = MockCardRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            auth_id: AuthId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let test_card = Card {
            id: CardId::new(1),
            account_id: AccountId::new(1),
            card_name: "suica".to_string(),
            card_number: [1, 16, 3, 16, 197, 20, 106, 38].to_vec(),
            created_at: Local::now().naive_local(),
        };
        let _ = account_repository.insert(&test_account);
        let _ = card_repository.insert(&test_card);

        let _ = delete_card(
            &card_repository,
            &account_repository,
            &test_card.id,
            &test_account.id,
        );

        assert!(get_card(
            &card_repository,
            &account_repository,
            &test_card.id,
            &test_account.id,
        )
        .is_err(),)
    }
}
