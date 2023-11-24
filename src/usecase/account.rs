use crate::domain::object::account::Account;
use crate::domain::repository::account::AccountRepository;
use anyhow::Result;

pub fn post_account(repository: &mut impl AccountRepository, account: &Account) -> Result<()> {
    repository.insert(account)
}

pub fn get_account_list(repository: impl AccountRepository) -> Result<Vec<Account>> {
    repository.list()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use std::cell::RefCell;
    use std::collections::HashMap;

    use anyhow::Result;

    struct MockAccountRepository {
        pool: RefCell<HashMap<u64, Account>>,
    }

    impl AccountRepository for MockAccountRepository {
        fn insert(&self, account: &Account) -> Result<()> {
            let _ = &self
                .pool
                .borrow_mut()
                .entry(1)
                .or_insert_with(|| account.clone());

            Ok(())
        }
        fn list(&self) -> Result<Vec<Account>> {
            let accounts: Vec<Account> = self.pool.borrow().values().cloned().collect();
            Ok(accounts)
        }
    }

    #[test]
    fn success_post_account() {
        let mut repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: 1,
            username: "test_user".to_string(),
            card_id: vec![1, 16, 3, 16, 197, 20, 106, 38],
            created_at: Local::now().naive_local(),
        };

        let result = post_account(&mut repository, &test_account);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    //     #[test]
    //     fn success_get_accounts() {
    //         let mut repository = MockAccountRepository {
    //             pool: RefCell::new(HashMap::new()),
    //         };

    //         let test_account = Account {
    //             id: 1,
    //             username: "test_user".to_string(),
    //             card_id: vec![1, 16, 3, 16, 197, 20, 106, 38],
    //             created_at: Local::now().naive_local(),
    //         };

    //         repository.insert(&test_account);

    //         // let result = get_document_list()
    //     }
}
