#[cfg(feature = "swagger")]
use actix_web::web;
#[cfg(feature = "swagger")]
use utoipa::OpenApi;
#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[cfg(feature = "swagger")]
#[derive(OpenApi)]
#[openapi(paths(
    _get_blog,
    _get_blogs_preview,
    _get_experince,
    _project,
    _skills_home,
    darkicewolf50_actix_setup::swagger_docs::health_check_swagger
))]
pub struct ApiDoc;

#[cfg(feature = "swagger")]
#[derive(ToSchema)]
pub struct _BlogContent {
    pub blog_file_name: String,
    pub date_last_edit: String,
    pub blog_title: String,
    pub tags: Vec<String>,
    pub html_blog_content: String,
}

#[cfg(feature = "swagger")]
#[utoipa::path(
    get,
    path = "/blogs/blog/{blog_name}",
    params(
        ("blog_name" = String, Path, description = "Name of the blog to retrieve")
    ),
    responses(
        (status = 200, description = "Blog Content, using the BlogContent schema", body = [_BlogContent])
    )
)]
pub async fn _get_blog(_blog_name: web::Path<String>) {}

#[cfg(feature = "swagger")]
#[utoipa::path(
    get,
    path = "/blogs/{num_limit}/{page_num}",
    params(
        ("num_limit" = u8, Path, description = "Number of blogs to get"),
        ("page_num" = u32, Path, description = "What multiple of that to get")
    ),
    responses(
        (status = 200, description = "Blog Preview, using the BlogPreview schema", body = [_BlogContent])
    )
)]
pub async fn _get_blogs_preview(_props: web::Path<(u8, u32)>) {}

#[cfg(feature = "swagger")]
#[derive(ToSchema)]
pub struct _TypeExp {
    experience_jobs: Vec<_ExpDes>,
    experience_vol: Vec<_ExpDes>,
}

#[cfg(feature = "swagger")]
#[derive(ToSchema)]
pub struct _ExpDes {
    pub postition: String,
    pub company: String,
    pub location: String,
    pub start_month: String,
    pub end_month: String,
}

#[cfg(feature = "swagger")]
#[utoipa::path(
    get,
    path = "/experience",

    responses(
        (status = 200, description = "Experience of what I hafve done for work, using the TypeExp schema", body = [_TypeExp])
    )
)]
pub async fn _get_experince() {}

#[cfg(feature = "swagger")]
#[derive(ToSchema)]
pub struct _TechDes {
    tech_name: String,
    tech_logo: String,
    project_site: String,
    #[schema(minimum = 0, maximum = 255)]
    skill_level: u8,
    #[schema(example = json!(["Backend"]))]
    tech_cat: Vec<String>,
}

#[cfg(feature = "swagger")]
#[utoipa::path(
    get,
    path = "/skills",
    responses(
        (status = 200, description = "Skill info, using the TechDes schema", body = [_TechDes])
    )
)]
pub async fn _skills_home() {}

#[cfg(feature = "swagger")]
#[derive(ToSchema)]
pub struct _ProjectDes {
    project_name: String,
    website_link: Option<String>,
    github_link: Option<String>,
    forgejo_link: Option<String>,
    dockerhub_link: Option<String>,
    project_img: Option<String>,
    techs_used: Vec<String>,
    project_des: String,
}

#[cfg(feature = "swagger")]
#[utoipa::path(
    get,
    path = "/projects/{num_limit}",
    params(
        ("num_limit" = usize, Path, description = "Number of projects to return, 0 for all")
    ),
    responses(
        (status = 200, description = "Project info, using the ProjectDes schema", body = [_ProjectDes])
    )
)]
pub async fn _project(_limit: web::Path<usize>) {}
