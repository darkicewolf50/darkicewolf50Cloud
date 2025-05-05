use actix_web::{HttpRequest, Responder, get, web};
// use actix_web::{HttpResponse, post};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;

pub fn log_incoming(req: HttpRequest, method: &str, path_source: &str) {
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
