mod envvars;

use actix_web::middleware::Logger;
use actix_web::post;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;

#[derive(serde::Serialize)]
#[serde(tag = "status", content = "data")]
pub enum HandlerResponse<T> {
    Success(T),
    Fail { code: u32, message: String },
}

/// Type aliases for clarity
type RequestData = Vec<u32>;
type ResponseData = u32;

#[post("/demo")]
async fn demo(_data: web::Data<Data>, body: web::Json<RequestData>) -> impl Responder {
    let list = body.into_inner();

    if list.is_empty() {
        let result = HandlerResponse::<ResponseData>::Fail {
            code: 1,
            message: "Empty list".to_string()
        };
        return HttpResponse::BadRequest().json(result);
    }

    let value: ResponseData = list.into_iter().sum();

    let result = HandlerResponse::<ResponseData>::Success(value);

    HttpResponse::Ok().json(result)
}

/// Application state
#[derive(Clone)]
struct Data {}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    let data = Data {};

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(data.clone()))
            .service(demo)
    })
    .bind(&*envvars::HOST)?
    .run()
    .await
    .map_err(anyhow::Error::from)
}
