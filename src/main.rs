use actix_web::{App, HttpServer, web};

use darkicewolf50_cloud::{hello, project, skills_home, test_reqwest};
// use darkicewolf50_cloud:: {echo, manual_hello, resend,};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            // .service(echo)
            // .service(resend)
            // .route("/hey", web::get().to(manual_hello))
            .service(project)
            .service(test_reqwest)
            .service(web::scope("/home").service(skills_home))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
