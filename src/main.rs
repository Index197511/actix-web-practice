use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

struct AppStateWithMessage {
    message: Mutex<String>,
}

fn add1(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); 
    *counter += 1;
    format!("Request number: {}", counter)
}

fn mul2(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter *= 2;
    format!("Request number: {}", counter)
}

fn say_hello(data: web::Data<AppStateWithMessage>) -> String{
    let mut message = data.message.lock().unwrap();
    *message = format!("{}{}", message, "hello ");
    format!("{}", message)
}

fn say_goodBye(data: web::Data<AppStateWithMessage>) -> String{
    let mut message = data.message.lock().unwrap();
    *message = format!("{}{}", message, "goodBye ");
    format!("{}", message)
}
fn main() {
    let counter = web::Data::new(AppStateWithCounter{
        counter: Mutex::new(0),
    });
    let message = web::Data::new(AppStateWithMessage {
        message: Mutex::new("init".to_string()),
    });

    HttpServer::new(move || {
        App::new()
            .register_data(counter.clone())
            .register_data(message.clone())
            .service(
                web::scope("/calc")
                    .route("/add", web::get().to(add1))
                    .route("/mul", web::get().to(mul2))
                )
            .service(
                web::scope("msg")
                    .route("/hello", web::get().to(say_hello))
                    .route("/goodbye", web::get().to(say_goodBye))
                )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
