use crate::server::connection::RequestContext;
use crate::server::request::auth::AuthRequest;
use crate::usecase;
use actix_web::{post, web, web::Json, HttpResponse, Responder};

#[post("/auth/signup")]
async fn signup(data: web::Data<RequestContext>, request: Json<AuthRequest>) -> impl Responder {
    match usecase::auth::signup(&data.auth_repository(), &request) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Internal Server Error {}", err))
        }
    }
}
