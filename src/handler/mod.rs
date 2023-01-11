use std::{fs::File, io::Read};
mod user;
use user::*;
mod auth;
use auth::*;

use actix_web::{web, web::ServiceConfig, HttpResponse};
use crate::errors::AppError;
type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut ServiceConfig) {
    let index = web::resource("/").route(web::get().to(index));
    let signup = web::resource("/signup").route(web::post().to(create_user));
    let me = web::resource("/me").route(web::get().to(me));
    let auth = web::resource("/auth").route(web::post().to(auth));
    config
        .service(index)
        .service(signup)
        .service(me)
        .service(auth);
}

pub async fn index() -> HttpResponse {
    let mut f = File::open("src/html/index.html").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer);
    HttpResponse::Ok().content_type("text/html").body(buffer)
}
