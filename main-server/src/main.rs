use crate::demo::hello;
use actix_web::{web, App, HttpServer};
mod card;
mod demo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            hello_scope()
        )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn hello_scope() -> actix_web::Scope {
    web::scope("/hello")
        .service(hello)
}

//访问视图  -> 视图调用schema api 、card api 组装视图内容，返回
fn view_scope() -> actix_web::Scope {
    web::scope("/view")
}