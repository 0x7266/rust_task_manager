use actix_web::{
    get, middleware::Logger, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .service(hw)
                        .service(echo_user)
                        .service(add_task),
                ),
            )
            .service(index)
            .default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", 3333))
    .expect("Failed to bind")
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("home")
}

#[get("echo/{user}")]
async fn echo_user(req: HttpRequest) -> impl Responder {
    let user = req.match_info().get("user").unwrap();
    HttpResponse::Ok().body(format!("hello, {}", user))
}

#[get("/hw")]
async fn hw() -> impl Responder {
    HttpResponse::Ok().body("hello, world!")
}

#[derive(Deserialize)]
struct Task {
    status: String,
    title: String,
}

#[post("/new")]
async fn add_task(task: web::Json<Task>) -> impl Responder {
    HttpResponse::Ok().body(format!("TASK: {}\nSTATUS: {}", task.title, task.status))
}

async fn not_found(req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().body("404 - Not Found")
}
