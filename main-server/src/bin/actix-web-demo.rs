use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// This struct represents state
struct AppState {
    app_name: String,
}
#[get("/app-name")]
async fn app_name(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create AppState
    let app_state = web::Data::new(AppState {
        app_name: String::from("Actix-Web"),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Pass app state to app
            .service(hello)
            .service(echo)
            .service(app_name)
            .route("/hey", web::get().to(manual_hello))
    })
        .workers(4)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}