use super::super::request::account::AccountRequest;
use super::super::response::account::AccountListResopnse;
use crate::server::RequestContext;
use crate::usecase;
use actix_web::{get, post, web, web::Json, HttpResponse, Responder};

#[post("/accounts")]
async fn post_account(
    data: web::Data<RequestContext>,
    request: Json<AccountRequest>,
) -> impl Responder {
    println!("a");
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
