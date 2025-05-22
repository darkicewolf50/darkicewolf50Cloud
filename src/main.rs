use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

use darkicewolf50_cloud::{get_blog, get_blogs_preview, hello, project, skills_home};
// use darkicewolf50_cloud:: {echo, manual_hello, resend,};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running on port 5050");
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    // Allow any origin — or use `.allowed_origin("http://localhost:8080")` to restrict
                    .allow_any_origin()
                    // Allow common methods
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    // Allow any headers
                    .allow_any_header(), // Optionally enable sending cookies, etc.
                                         //.supports_credentials()
            )
            .service(hello)
            // .service(echo)
            // .service(resend)
            // .route("/hey", web::get().to(manual_hello))
            .service(project)
            .service(
                web::scope("/blogs")
                    .service(get_blog)
                    .service(get_blogs_preview),
            )
            .service(web::scope("/home").service(skills_home))
    })
    .bind(("0.0.0.0", 5050))?
    .run()
    .await
}
