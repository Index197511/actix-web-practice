use actix_web::{get, post, App, HttpServer, HttpResponse, Responder};
 
#[get("/get")]
fn get() -> impl Responder {
    HttpResponse::Ok().body("GET")
}
 
#[post("/post")]
fn post(a: String) -> (){
    println!("POST");
    println!("{}", a);
}

 
fn main() {
    HttpServer::new(
        || App::new()
            .service(get)
            .service(post)
        )
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
