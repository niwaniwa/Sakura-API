use super::super::request::account::AccountRequest;
use crate::server::RequestContext;
use crate::usecase;
use actix_web::{post, web, web::Json, HttpResponse, Responder};

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
