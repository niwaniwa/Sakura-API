use crate::domain::object::account::{Account, AccountId};
use crate::domain::repository::account::AccountRepository;
use anyhow::Result;

pub fn post_account(repository: &mut impl AccountRepository, account: &Account) -> Result<()> {
    repository.insert(account)
}

pub fn get_account_list(repository: &mut impl AccountRepository) -> Result<Vec<Account>> {
    repository.list()
}

pub fn get_account(
    repository: &mut impl AccountRepository,
    account_id: &AccountId,
) -> Result<Account> {
    repository.find_by_id(account_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::object::account::AccountId;
    use chrono::Local;
    use std::cell::RefCell;
    use std::collections::HashMap;

    use anyhow::Result;

    struct MockAccountRepository {
        pool: RefCell<HashMap<i64, Account>>,
    }

    impl AccountRepository for MockAccountRepository {
        fn insert(&self, account: &Account) -> Result<()> {
            let _ = &self
                .pool
                .borrow_mut()
                .entry(account.id.get())
                .or_insert_with(|| account.clone());

            Ok(())
        }
        fn list(&self) -> Result<Vec<Account>> {
            let accounts: Vec<Account> = self.pool.borrow().values().cloned().collect();
            Ok(accounts)
        }

        fn find_by_id(&self, account_id: &AccountId) -> Result<Account> {
            match self.pool.borrow().get(&account_id.get()) {
                Some(account) => Ok(account.clone()),
                None => Err(anyhow::anyhow!("Account not found")),
            }
        }
    }

    #[test]
    fn success_post_account() {
        let mut repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            card_type: "Suica".to_string(),
            card_id: vec![1, 16, 3, 16, 197, 20, 106, 38],
            created_at: Local::now().naive_local(),
        };

        let result = post_account(&mut repository, &test_account);
        assert!(result.is_ok());
    }

    #[test]
    fn success_get_accounts() {
        let mut repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            card_type: "Suica".to_string(),
            card_id: vec![1, 16, 3, 16, 197, 20, 106, 38],
            created_at: Local::now().naive_local(),
        };

        let test_account2 = Account {
            id: AccountId::new(2),
            username: "test_user2".to_string(),
            grade: 4,
            card_type: "Suica".to_string(),
            card_id: vec![1, 16, 3, 16, 197, 20, 106, 38],
            created_at: Local::now().naive_local(),
        };

        let _ = repository.insert(&test_account);
        let _ = repository.insert(&test_account2);

        let result = get_account_list(&mut repository);

        let accounts = result.unwrap();
        assert_eq!(accounts.len(), 2);
    }

    #[test]
    fn success_find_account_by_id() {
        let mut repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            card_type: "Suica".to_string(),
            card_id: vec![1, 16, 3, 16, 197, 20, 106, 38],
            created_at: Local::now().naive_local(),
        };

        let _ = repository.insert(&test_account);

        let result = get_account(&mut repository, &test_account.id);

        assert!(result.is_ok());

        let retrieved_account = result.unwrap();

        assert_eq!(retrieved_account.id.get(), test_account.id.get());
        assert_eq!(retrieved_account.username, test_account.username);
        assert_eq!(retrieved_account.grade, test_account.grade);
        assert_eq!(retrieved_account.card_type, test_account.card_type);
        assert_eq!(retrieved_account.card_id, test_account.card_id);
        assert_eq!(retrieved_account.created_at, test_account.created_at);
    }

    #[test]
    fn failed_find_account_by_id() {
        let mut repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            card_type: "Suica".to_string(),
            card_id: vec![1, 16, 3, 16, 197, 20, 106, 38],
            created_at: Local::now().naive_local(),
        };

        let _ = repository.insert(&test_account);

        let result = get_account(&mut repository, &AccountId::new(2));

        assert!(result.is_err());
    }
}
