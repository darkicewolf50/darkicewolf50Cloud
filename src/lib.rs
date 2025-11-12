use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder, get, web};
// use actix_web::HttpResponse;
use comrak::{Options, markdown_to_html};
use darkicewolf50_actix_setup::{clean_user_file_req, log_incoming};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_yaml_bw;
use std::{fs, path::Path};
use time::{OffsetDateTime, format_description::well_known::Iso8601};

#[cfg(feature = "swagger")]
pub use swagger_docs::ApiDoc;

#[cfg(feature = "swagger")]
#[allow(dead_code)]
pub mod swagger_docs;

#[derive(Deserialize, Serialize, Debug)]
struct TechDes {
    tech_name: String,
    tech_logo: String,
    project_site: String,
    skill_level: u8,
    #[serde(default)]
    tech_cat: Vec<String>,
}

#[get("/skills")]
pub async fn skills_home() -> impl Responder {
    log_incoming("GET", "/skills");
    let raw_yaml: String = fs::read_to_string("./database/skill_level.yaml").unwrap();
    // .expect("Cannot open file or missing file.");
    let vec_yaml: Vec<TechDes> = serde_yaml_bw::from_str(&raw_yaml).unwrap_or_else(|_| vec![]);

    web::Json(vec_yaml)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ProjectDes {
    project_name: String,
    website_link: Option<String>,
    github_link: Option<String>,
    forgejo_link: Option<String>,
    dockerhub_link: Option<String>,
    project_img: Option<String>,
    techs_used: Vec<String>,
    project_des: String,
}

#[get("/projects/{num_limit}")]
pub async fn project(limit: web::Path<usize>) -> impl Responder {
    log_incoming("GET", "/projects/{num_limit}");

    let limit = limit.into_inner();

    let raw_yaml: String = fs::read_to_string("./database/projects.yaml").unwrap();
    let vec_yaml: Vec<ProjectDes> = serde_yaml_bw::from_str(&raw_yaml).unwrap_or_else(|_| vec![]);

    let res_vec: Vec<ProjectDes> = if limit == 0 || limit >= vec_yaml.len() {
        vec_yaml
    } else {
        vec_yaml[..limit].to_vec()
    };

    web::Json(res_vec)
}

#[derive(Deserialize, Serialize, Debug)]
struct BlogContent {
    pub blog_file_name: String,
    pub date_last_edit: String,
    pub blog_title: String,
    pub tags: Vec<String>,
    pub html_blog_content: String,
}

#[get("/blog/{blog_name}")]
pub async fn get_blog(blog_name: web::Path<String>) -> impl Responder {
    log_incoming("GET", "/blogs/blog/{blog_name}");

    let blog_name = blog_name.into_inner();
    let path = match clean_user_file_req("./blogs", &blog_name, "md") {
        Ok(p) => p,
        Err(_) => {
            return web::Json(BlogContent {
                blog_file_name: String::new(),
                date_last_edit: "9999-12-01".to_string(),
                blog_title: "Not Found".to_string(),
                tags: vec!["#error".to_string()],
                html_blog_content: "<p>Blog not found</p>".to_string(),
            });
        }
    };

    let Ok(blog_text) = fs::read_to_string(&path) else {
        return web::Json(BlogContent {
            blog_file_name: String::new(),
            date_last_edit: "9999-12-01".to_string(),
            blog_title: "Not Found".to_string(),
            tags: vec!["#error".to_string()],
            html_blog_content: "<p>Blog not found</p>".to_string(),
        });
    };
    let mut blog_lines = blog_text.lines();

    let raw_title = blog_lines.next().unwrap_or("").trim();
    let blog_title = raw_title
        .strip_prefix("# ")
        .unwrap_or(raw_title)
        .to_string();

    //consumes empty line
    blog_lines.next();
    let tags = blog_lines
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let markdown_content: String = blog_lines.collect::<Vec<_>>().join("\n");

    // Allow, images and embeds
    let mut options = Options::default();
    options.extension.table = true; // Enable GitHub-style tables
    options.extension.strikethrough = true; // Enable strikethrough
    options.parse.smart = true;
    options.render.github_pre_lang = true;
    options.render.r#unsafe = true;

    let html_blog = markdown_to_html(&markdown_content, &options);
    let date_last_edit = get_date_modified(&path).unwrap_or_else(|| "".to_string());

    web::Json(BlogContent {
        blog_file_name: blog_name,
        blog_title: blog_title,
        tags: tags,
        html_blog_content: html_blog,
        date_last_edit: date_last_edit,
    })
}

#[get("/{num_limit}/{page_num}")]
pub async fn get_blogs_preview(props: web::Path<(u8, u32)>) -> impl Responder {
    log_incoming("GET", "blogs/{num_limit}/{page_num}");

    let (num_limit, page_num) = props.into_inner();

    let mut available_blogs: Vec<String> = Vec::new();
    let dir = Path::new("./blogs");
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry
                .unwrap()
                .file_name()
                .into_string()
                .unwrap_or("".to_string());
            let _ = &available_blogs.push(entry);
        }
    }

    let start: usize = page_num as usize * num_limit as usize;
    let end: usize = start + num_limit as usize;

    if start >= available_blogs.len().try_into().unwrap_or(0) {
        return web::Json(serde_json::json!("Blogs: []"));
    }

    let end = end.min(available_blogs.len());
    let blogs_to_get = &available_blogs[start..end];
    let mut blogs_preview: Vec<BlogContent> = Vec::new();

    for blog_info in blogs_to_get {
        let path = dir.join(blog_info);

        let date_last_edit = get_date_modified(&path).unwrap();

        let raw_blog_string = fs::read_to_string(&path).unwrap_or("".to_string());
        let mut raw_blog = raw_blog_string.lines();

        let raw_title = raw_blog.next().unwrap_or("").trim();
        let title = raw_title
            .strip_prefix("# ")
            .unwrap_or(raw_title)
            .to_string();
        raw_blog.next();
        let tags = raw_blog
            .next()
            .unwrap_or("")
            .trim()
            .to_string()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        raw_blog.next();
        let raw_blog_preview = format!("{}...", raw_blog.next().unwrap_or(""));
        let blog_preview = comrak::markdown_to_html(&raw_blog_preview, &comrak::Options::default());

        blogs_preview.push(BlogContent {
            blog_file_name: blog_info.strip_suffix(".md").unwrap().to_string(),
            date_last_edit: date_last_edit,
            blog_title: title,
            tags: tags,
            html_blog_content: blog_preview,
        });
    }

    web::Json(serde_json::json!(blogs_preview))
}

fn get_date_modified(path: &Path) -> Option<String> {
    let metadata = fs::metadata(path).ok()?;
    let system_time = metadata.modified().ok()?;
    let offset_time = OffsetDateTime::from(system_time);

    // Format just the date part of ISO 8601 (e.g. "2025-05-15")
    let iso_string = offset_time.format(&Iso8601::DEFAULT).ok()?;
    let date_only = iso_string.split('T').next()?.to_string();

    Some(date_only)
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//         test_reqwest();
//     }
// }

#[derive(Debug, Deserialize)]
struct TypeExp {
    #[serde(rename = "EXPERIENCE_JOBS")]
    experience_jobs: Vec<ExpDes>,
    #[serde(rename = "EXPERIENCE_VOL")]
    experience_vol: Vec<ExpDes>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ExpDes {
    pub postition: String,
    pub company: String,
    pub location: String,
    pub start_month: String,
    pub end_month: String,
}

#[get("/experience")]
pub async fn get_experince() -> impl Responder {
    log_incoming("GET", "/experience");
    let raw_yaml: String = fs::read_to_string("./database/experience.yaml").unwrap();
    let read_yaml: Result<TypeExp, _> = serde_yaml_bw::from_str(&raw_yaml);

    let parsed_yaml = read_yaml.unwrap_or(TypeExp {
        experience_jobs: Vec::new(),
        experience_vol: Vec::new(),
    });

    web::Json(json!({
    "body": {
            "EXPERIENCE_JOBS": parsed_yaml.experience_jobs,
            "EXPERIENCE_VOL": parsed_yaml.experience_vol
        }
    }))
}

#[get("/{static_file}")]
pub async fn get_static_file(
    static_file: web::Path<String>,
) -> actix_web::Either<actix_web::Result<NamedFile>, HttpResponse> {
    log_incoming("GET", "/static_file");

    let file_string = static_file.into_inner();
    let mut file_parts = file_string.rsplitn(2, ".");
    // let mut parts_iter = file_thing.rsplitn(2, '.');
    let (file_ext, file_name) = match (file_parts.next(), file_parts.next()) {
        (Some(ext), Some(name)) => (ext, name),
        _ => {
            return actix_web::Either::Right(
                HttpResponse::BadRequest().body("Invalid file request"),
            );
        }
    };

    let file_path = match clean_user_file_req("./static", file_name, file_ext) {
        Ok(path) => path,
        Err(e) => return actix_web::Either::Right(e),
    };

    match NamedFile::open(&file_path) {
        Ok(named_file) => actix_web::Either::Left(Ok(named_file)),
        Err(_) => actix_web::Either::Right(HttpResponse::NotFound().finish()),
    }
}
