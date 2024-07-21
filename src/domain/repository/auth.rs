use super::super::object::auth::Auth;
use anyhow;

pub trait AuthRepository {
    fn signup(&self, auth: &Auth) -> anyhow::Result<()>;
}
