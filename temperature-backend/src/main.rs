use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ConversionRequest {
    value: f64,
    from_unit: String,
    to_unit: String,
}

#[derive(Serialize)]
struct ConversionResponse {
    result: f64,
}

async fn convert(req: web::Json<ConversionRequest>) -> impl Responder {
    let result = match (&*req.from_unit, &*req.to_unit) {
        ("Celsius", "Fahrenheit") => req.value * 1.8 + 32.0,
        ("Fahrenheit", "Celsius") => (req.value - 32.0) / 1.8,
        ("Celsius", "Kelvin") => req.value + 273.15,
        ("Kelvin", "Celsius") => req.value - 273.15,
        ("Fahrenheit", "Kelvin") => (req.value - 32.0) / 1.8 + 273.15,
        ("Kelvin", "Fahrenheit") => (req.value - 273.15) * 1.8 + 32.0,
        _ => return HttpResponse::BadRequest().body("Unsupported units"),
    };
    HttpResponse::Ok().json(ConversionResponse { result })
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ready and Alive")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
            .service(web::resource("/api/convert").route(web::post().to(convert)))
            .service(web::resource("/api/health").route(web::get().to(health)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}