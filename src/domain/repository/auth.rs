use super::super::object::auth::Auth;
use anyhow;

pub trait AuthRepository {
    // fn signup(&self, auth: &Auth) -> anyhow::Result<()>;
    fn exist_by_email(&self, email: &str) -> anyhow::Result<bool>;
}
