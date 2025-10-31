use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

use darkicewolf50_actix_setup::health_check;
use darkicewolf50_cloud::ApiDoc;
use darkicewolf50_cloud::{get_blog, get_blogs_preview, get_experince, project, skills_home};
use utoipa::OpenApi;
#[cfg(debug_assertions)]
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running on port 5050");

    HttpServer::new(|| {
        let app = App::new()
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
            .service(health_check)
            .service(project)
            .service(
                web::scope("/blogs")
                    .service(get_blog)
                    .service(get_blogs_preview),
            )
            .service(
                web::scope("/home")
                    .service(skills_home)
                    .service(get_experince),
            );

        // swagger ui only available in debug mode
        // available at the /swagger-ui route
        #[cfg(debug_assertions)]
        let app = app.service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        );
            
        app
    })
    .bind(("0.0.0.0", 5050))?
    .run()
    .await
}
