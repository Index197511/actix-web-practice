use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again")
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index))
                    .route("index2.html", web::get().to(index2))
                )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
