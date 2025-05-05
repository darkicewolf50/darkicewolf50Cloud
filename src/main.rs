use actix_web::{App, HttpServer, web};

use darkicewolf50_cloud::{echo, hello, manual_hello, resend, skills_home};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(resend)
            .route("/hey", web::get().to(manual_hello))
            .service(web::scope("/home").service(skills_home))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
