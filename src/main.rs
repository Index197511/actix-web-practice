use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); 
    *counter += 1;
    format!("Request number: {}", counter)
}

fn index2(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter *= 2;
    format!("Request number: {}", counter)
}

fn main() {
    let counter = web::Data::new(AppStateWithCounter{
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .register_data(counter.clone())
            .service(
                web::scope("/app")
                    .route("/add", web::get().to(index))
                    .route("/mul", web::get().to(index2))
                )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
