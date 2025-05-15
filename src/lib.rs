use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs, path::Path};
use time::{OffsetDateTime, format_description::well_known::Iso8601};

pub fn log_incoming(req: HttpRequest, method: &'static str, path_source: &str) {
    let peer_addr = req.peer_addr();
    if let Some(ip_addr_other) = peer_addr {
        println!(
            "{} request from: {}, subaddress: {}",
            method, ip_addr_other, path_source
        );
    } else {
        println!(
            "{} request from: unknown, subaddress: {}",
            method, path_source
        );
    }
}

#[get("/")]
pub async fn hello(req: HttpRequest) -> impl Responder {
    log_incoming(req, "GET", "/");
    web::Json(json!({
    "body": {
            "message": "Hello I am alive, this does nothing"
        }
    }))
}

#[derive(Deserialize, Serialize, Debug)]
struct TechDes {
    tech_name: String,
    tech_logo: String,
    project_site: String,
    skill_level: u8,
    tech_cat: Vec<String>,
}

#[get("/skills")]
pub async fn skills_home(req: HttpRequest) -> impl Responder {
    log_incoming(req, "GET", "/skills");
    let raw_yaml: String = fs::read_to_string("./data_txt/skill_level.yaml").unwrap();
    // .expect("Cannot open file or missing file.");
    let vec_yaml = yaml_rust2::YamlLoader::load_from_str(&raw_yaml).unwrap()[0].clone();

    let res_vec: Vec<TechDes> = vec_yaml
        .as_vec()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| TechDes {
            tech_name: item["tech_name"].as_str().unwrap_or("").to_string(),
            tech_logo: item["tech_logo"].as_str().unwrap_or("").to_string(),
            project_site: item["project_site"].as_str().unwrap_or("").to_string(),
            skill_level: item["skill_level"].as_i64().unwrap_or(0) as u8,
            tech_cat: item["tech_cat"]
                .as_vec()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|item_cat| item_cat.as_str().map(|inner_cat| inner_cat.to_string()))
                .collect(),
        })
        .collect();
    // println!("{:#?}", res_vec[2]);
    web::Json(res_vec)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ProjectDes {
    project_name: String,
    website_link: String,
    github_link: String,
    project_img: String,
    techs_used: Vec<String>,
    project_des: String,
}

#[get("/projects/{num_limit}")]
pub async fn project(limit: web::Path<usize>, req: HttpRequest) -> impl Responder {
    log_incoming(req, "GET", "/projects/{num_limit}");

    let limit = limit.into_inner();

    let raw_yaml: String = fs::read_to_string("./data_txt/projects.yaml").unwrap();
    let vec_yaml = yaml_rust2::YamlLoader::load_from_str(&raw_yaml).unwrap()[0].clone();

    let raw_vec: Vec<ProjectDes> = vec_yaml
        .as_vec()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| ProjectDes {
            project_name: item["project_name"].as_str().unwrap_or("").to_string(),
            website_link: item["website_link"].as_str().unwrap_or("").to_string(),
            github_link: item["github_link"].as_str().unwrap_or("").to_string(),
            project_img: item["project_img"].as_str().unwrap_or("").to_string(),
            techs_used: item["techs_used"]
                .as_vec()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|item_str| item_str.as_str().map(|inner_item| inner_item.to_string()))
                .collect(),
            project_des: item["project_des"].as_str().unwrap_or("").to_string(),
        })
        .collect();
    let res_vec: Vec<ProjectDes> = if limit == 0 || limit >= raw_vec.len() {
        raw_vec
    } else {
        raw_vec[..limit].to_vec()
    };

    web::Json(res_vec)
}

// {how_many} how_many: {}  how_many,
#[get("/blog/{blog_name}")]
pub async fn get_blog(
    blog_name: web::Path<String>,
    // how_many: web::Path<i32>,
    req: HttpRequest,
) -> impl Responder {
    log_incoming(req, "GET", "/blogs/blog/{blog_name}");
    let blog = fs::read_to_string(format!("./data_txt/{blog_name}.md")).unwrap_or("".to_string());
    let html_blog = comrak::markdown_to_html(&blog, &comrak::Options::default());
    HttpResponse::Ok().body(html_blog)
}

#[derive(Deserialize, Serialize, Debug)]
struct BlogPreview {
    pub blog_file_name: String,
    pub date_last_edit: String,
    pub html_preview: String,
}

#[get("/{num_limit}/{page_num}")]
pub async fn get_blogs_preview(props: web::Path<(u8, u32)>, req: HttpRequest) -> impl Responder {
    log_incoming(req, "GET", "blogs/{num_limit}/{page_num}");

    let (num_limit, page_num) = props.into_inner();

    let mut available_blogs: Vec<String> = Vec::new();
    let dir = Path::new("./data_txt/blogs");
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
    let mut blogs_preview: Vec<BlogPreview> = Vec::new();

    for blog_info in blogs_to_get {
        let path = dir.join(blog_info);

        let date_last_edit = get_date_modified(&path).unwrap();

        let blog = fs::read_to_string(&path).unwrap_or("".to_string());
        let raw_blog = blog
            .lines()
            .filter(|line| !line.trim().is_empty())
            .take(3)
            .collect::<Vec<_>>()
            .join("\n");
        let html_blog = comrak::markdown_to_html(&raw_blog, &comrak::Options::default());

        blogs_preview.push(BlogPreview {
            blog_file_name: blog_info.to_string(),
            date_last_edit: date_last_edit,
            html_preview: html_blog,
        });
    }

    // println!("{:?}", blogs_preview);
    // web::Json(serde_json::json!(
    //     { "Hello there!": "General Kenobi", "Available in dir": format!("{:?}",blogs_to_get) }
    // ))
    web::Json(serde_json::json!({ "Blogs": blogs_preview }))
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
