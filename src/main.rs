use actix_cors::Cors;
use actix_web::{App, HttpRequest, HttpServer, web};
use darkicewolf50_actix_setup::{health_check, log_incoming_w_x};
use darkicewolf50_cloud::{
    get_blog, get_blogs_preview, get_experince, get_static_file, project, skills_home,
};

#[cfg(feature = "swagger")]
use darkicewolf50_cloud::swagger_docs::ApiDoc;
#[cfg(feature = "swagger")]
use utoipa::OpenApi;
#[cfg(feature = "swagger")]
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    {
        println!("Running on http://localhost:5050/");
    }
    #[cfg(not(debug_assertions))]
    {
        println!("Running on port 5050");
    }

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
            )
            .service(web::scope("/static").service(get_static_file))
            .service(get_robots)
            .service(get_favicon_ico)
            .service(get_favicon_png);

        // swagger ui only available in debug mode
        // available at the /swagger-ui route
        #[cfg(feature = "swagger")]
        let app = app.service(
            SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        );

        app
    })
    .bind(("0.0.0.0", 5050))?
    .run()
    .await
}

#[actix_web::get("/robots.txt")]
pub async fn get_robots(req: HttpRequest) -> impl actix_web::Responder {
    log_incoming_w_x("GET", "/robots.txt", req);
    actix_files::NamedFile::open("./static/robots.txt")
        .map(|f| f.use_last_modified(true))
        .map_err(|_| actix_web::error::ErrorNotFound("robots.txt not found"))
}

#[actix_web::get("/favicon.ico")]
pub async fn get_favicon_ico(req: HttpRequest) -> impl actix_web::Responder {
    log_incoming_w_x("GET", "/favicon.ico", req);
    actix_files::NamedFile::open("./static/favicon.ico")
        .map(|f| f.use_last_modified(true))
        .map_err(|_| actix_web::error::ErrorNotFound("favicon.ico not found"))
}

#[actix_web::get("/favicon.png")]
pub async fn get_favicon_png(req: HttpRequest) -> impl actix_web::Responder {
    log_incoming_w_x("GET", "/favicon.png", req);
    actix_files::NamedFile::open("./static/favicon.png")
        .map(|f| f.use_last_modified(true))
        .map_err(|_| actix_web::error::ErrorNotFound("favicon.png not found"))
}
