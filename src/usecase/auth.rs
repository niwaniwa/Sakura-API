use crate::domain::object::auth::Auth;
use crate::domain::repository::auth::AuthRepository;
use crate::server::request::auth::AuthRequest;
use anyhow;

pub fn signup(repository: &impl AuthRepository, request: &AuthRequest) -> anyhow::Result<()> {
    Auth::validate(&request.email, &request.email)?;

    let hash_password = Auth::hash_password(&request.password)?;

    let auth = Auth::new(&request.email, &hash_password);

    repository.signup(&auth)
}
