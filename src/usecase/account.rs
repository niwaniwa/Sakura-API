use crate::domain::object::account::{Account, AccountId};
use crate::domain::repository::account::AccountRepository;
use crate::server::request::account::AccountRequest;
use actix_web::web::Json;
use anyhow;

pub fn post_account(repository: &impl AccountRepository, account: &Account) -> anyhow::Result<()> {
    repository.insert(account)
}

pub fn get_account_list(repository: &impl AccountRepository) -> anyhow::Result<Vec<Account>> {
    repository.list()
}

pub fn get_account(
    repository: &impl AccountRepository,
    account_id: &AccountId,
) -> anyhow::Result<Account> {
    repository.find_by_id(account_id)
}

pub fn put_account(
    repository: &impl AccountRepository,
    request: &Json<AccountRequest>,
    account_id: &AccountId,
) -> anyhow::Result<()> {
    let account = repository.find_by_id(account_id)?;
    let updated_account = request.model(account.id, account.created_at);
    repository.update(&updated_account)
}

pub fn delete_account(
    repository: &impl AccountRepository,
    account_id: &AccountId,
) -> anyhow::Result<()> {
    let account = repository.find_by_id(account_id)?;
    repository.delete(&account)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::object::account::AccountId;
    use crate::domain::object::auth::AuthId;
    use crate::domain::object::card::{Card, CardId};
    use crate::domain::repository::card::CardRepository;
    use crate::tests::mock_account_repository::MockAccountRepository;
    use crate::tests::mock_card_repository::MockCardRepository;
    use chrono::{Duration, Local};
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[test]
    fn success_post_account() {
        let repository = MockAccountRepository {
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

        let result = post_account(&repository, &test_account);
        assert!(result.is_ok());
    }

    #[test]
    fn success_get_accounts() {
        let repository = MockAccountRepository {
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

        let test_account2 = Account {
            id: AccountId::new(2),
            auth_id: AuthId::new(2),
            username: "test_user2".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let _ = repository.insert(&test_account);
        let _ = repository.insert(&test_account2);

        let result = get_account_list(&repository);

        let accounts = result.unwrap();
        assert_eq!(accounts.len(), 2);
    }

    #[test]
    fn success_find_account_by_id() {
        let repository = MockAccountRepository {
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

        let _ = repository.insert(&test_account);

        let result = get_account(&repository, &test_account.id);

        assert!(result.is_ok());

        let retrieved_account = result.unwrap();

        assert_eq!(retrieved_account.id.get(), test_account.id.get());
        assert_eq!(retrieved_account.auth_id.get(), test_account.auth_id.get());
        assert_eq!(retrieved_account.username, test_account.username);
        assert_eq!(retrieved_account.grade, test_account.grade);
        assert_eq!(
            retrieved_account.expiration_date,
            test_account.expiration_date
        );
        assert_eq!(retrieved_account.created_at, test_account.created_at);
    }

    #[test]
    fn failed_find_account_by_id() {
        let repository = MockAccountRepository {
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

        let _ = repository.insert(&test_account);

        let result = get_account(&repository, &AccountId::new(2));

        assert!(result.is_err());
    }

    #[test]
    fn success_put_account() {
        let repository = MockAccountRepository {
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

        let update_account = AccountRequest {
            auth_id: AuthId::new(1),
            username: "update_user".to_string(),
            grade: 3,
            expiration_date: Local::now().naive_local() + Duration::hours(2),
        };

        let _ = repository.insert(&test_account);
        let result = put_account(&repository, &Json(update_account), &test_account.id);
        assert!(result.is_ok());
    }

    #[test]
    fn success_delete_account() {
        let repository = MockAccountRepository {
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

        let _ = post_account(&repository, &test_account);

        let _ = delete_account(&repository, &test_account.id);

        assert!(get_account(&repository, &test_account.id).is_err())
    }
    #[test]
    fn success_delete_account_and_tied_card() {
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

        let _ = delete_account(&account_repository, &test_account.id);

        assert!(account_repository.find_by_id(&test_account.id).is_err());
        // assert!(card_repository
        //     .find_by_id(&test_card.id, &test_account.id)
        //     .is_err());
    }
}
