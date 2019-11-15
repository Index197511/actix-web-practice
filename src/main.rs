use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again")
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
        .service(
            web::scope("/app")
                .route("/return", web::get().to(index))
            )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
