use actix_web::{web, App, HttpServer, HttpResponse, middleware};
 
fn get() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("index.html"))
}

 
fn post(a: String) -> (){
    println!("POST");
    println!("{}", a);
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::resource("/game").route(web::get().to(get)))
            .service(web::resource("/post").route(web::post().to(post)))
        );
}
 
fn main() {
    HttpServer::new(
        || App::new()
            .wrap(middleware::Logger::default())
            .configure(app_config)
        )
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
