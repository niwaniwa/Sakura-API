use crate::domain::object::account::Account;
use crate::domain::repository::account::AccountRepository;
use anyhow::Result;

pub fn post_account(repository: &mut impl AccountRepository, account: &Account) -> Result<()> {
    repository.insert(account)
}

#[cfg(test)]
mod tests {
    use super::*;
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
        };

        let result = post_account(&mut repository, &test_account);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
