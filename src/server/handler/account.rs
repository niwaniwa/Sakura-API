use super::super::request::account::AccountRequest;
use super::super::response::account::{AccountDto, AccountListResopnse};
use crate::domain::object::account::AccountId;
use crate::server::RequestContext;
use crate::usecase;
use actix_web::{delete, get, post, web, web::Json, HttpResponse, Responder};

#[post("/accounts")]
async fn post_account(
    data: web::Data<RequestContext>,
    request: Json<AccountRequest>,
) -> impl Responder {
    match usecase::account::post_account(&mut data.account_repository(), &request.of()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[get("/accounts")]
async fn get_accounts(data: web::Data<RequestContext>) -> impl Responder {
    match usecase::account::get_account_list(&mut data.account_repository()) {
        Ok(accounts) => HttpResponse::Ok().json(AccountListResopnse::new(accounts)),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[get("/accounts/{id}")]
async fn get_account(
    data: web::Data<RequestContext>,
    path_params: web::Path<(i64,)>,
) -> impl Responder {
    let account_id = AccountId::new(path_params.into_inner().0);
    match usecase::account::get_account(&mut data.account_repository(), &account_id) {
        Ok(account) => HttpResponse::Ok().json(AccountDto::new(&account)),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[delete("/accounts/{id}")]
async fn delete_account(
    data: web::Data<RequestContext>,
    path_params: web::Path<(i64,)>,
) -> impl Responder {
    let account_id = AccountId::new(path_params.into_inner().0);
    match usecase::account::delete_account(&mut data.account_repository(), &account_id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}
