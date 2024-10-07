use actix_web::{get, HttpResponse, post, Responder, web};

use crate::server::connection::RequestContext;
use crate::server::response::mqtt_card::MqttCardIdResponse;
use crate::server::response::register::IsRegisterResponse;
use crate::usecase;

#[post("/register")]
async fn register(data: web::Data<RequestContext>) -> impl Responder {
    match usecase::register::start_register(&data.register_repository()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Internal Server Error {}", err))
        }
    }
}

#[get("/register")]
async fn is_register(data: web::Data<RequestContext>) -> impl Responder {
    match usecase::register::is_register(&data.register_repository()) {
        Ok(mode) => {
            let response = IsRegisterResponse::new(mode);
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Internal Server Error {}", err))
        }
    }
}

#[get("/register/card")]
async fn get_card(data: web::Data<RequestContext>) -> impl Responder {
    match usecase::register::get_card(&data.register_repository()) {
        Ok(card_id) => HttpResponse::Ok().json(MqttCardIdResponse::new(card_id)),
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Internal Server Error {}", err))
        }
    }
}
