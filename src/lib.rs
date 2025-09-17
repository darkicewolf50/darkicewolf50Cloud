use actix_web::{HttpRequest, Responder, get, web};
// use actix_web::HttpResponse;
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
    let raw_yaml: String = fs::read_to_string("/database/skill_level.yaml").unwrap();
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
    web::Json(res_vec)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ProjectDes {
    project_name: String,
    website_link: String,
    github_link: String,
    forgejo_link: String,
    dockerhub_link: String,
    project_img: String,
    techs_used: Vec<String>,
    project_des: String,
}

#[get("/projects/{num_limit}")]
pub async fn project(limit: web::Path<usize>, req: HttpRequest) -> impl Responder {
    log_incoming(req, "GET", "/projects/{num_limit}");

    let limit = limit.into_inner();

    let raw_yaml: String = fs::read_to_string("/database/projects.yaml").unwrap();
    let vec_yaml = yaml_rust2::YamlLoader::load_from_str(&raw_yaml).unwrap()[0].clone();

    let raw_vec: Vec<ProjectDes> = vec_yaml
        .as_vec()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| ProjectDes {
            project_name: item["project_name"].as_str().unwrap_or("").to_string(),
            website_link: item["website_link"].as_str().unwrap_or("").to_string(),
            github_link: item["github_link"].as_str().unwrap_or("").to_string(),
            forgejo_link: item["forgejo_link"].as_str().unwrap_or("").to_string(),
            dockerhub_link: item["dockerhub_link"].as_str().unwrap_or("").to_string(),
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

#[derive(Deserialize, Serialize, Debug)]
struct BlogContent {
    pub blog_file_name: String,
    pub date_last_edit: String,
    pub blog_title: String,
    pub tags: Vec<String>,
    pub html_blog_content: String,
}

// {how_many} how_many: {}  how_many,
#[get("/blog/{blog_name}")]
pub async fn get_blog(
    blog_name: web::Path<String>,
    // how_many: web::Path<i32>,
    req: HttpRequest,
) -> impl Responder {
    log_incoming(req, "GET", "/blogs/blog/{blog_name}");
    let blog_name = blog_name.into_inner();
    let file_path = format!("/blogs/{}.md", blog_name);
    let path = Path::new(&file_path);

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
    let html_blog = comrak::markdown_to_html(&markdown_content, &comrak::Options::default());
    let date_last_edit = get_date_modified(path).unwrap_or_else(|| "".to_string());

    web::Json(BlogContent {
        blog_file_name: blog_name,
        blog_title: blog_title,
        tags: tags,
        html_blog_content: html_blog,
        date_last_edit: date_last_edit,
    })
}

#[get("/{num_limit}/{page_num}")]
pub async fn get_blogs_preview(props: web::Path<(u8, u32)>, req: HttpRequest) -> impl Responder {
    log_incoming(req, "GET", "blogs/{num_limit}/{page_num}");

    let (num_limit, page_num) = props.into_inner();

    let mut available_blogs: Vec<String> = Vec::new();
    let dir = Path::new("/blogs");
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
