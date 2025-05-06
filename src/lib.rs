use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
// use actix_web::{HttpResponse, post};
use reqwest::Client;
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

// a successful version of a webhook
// #[get("/test_reqwest")]
// pub async fn test_reqwest(req: HttpRequest) -> impl Responder {
//     log_incoming(req, "GET", "/test_reqwest");
//     let json_to_send = serde_json::json!({
//       "content": "Hey, from Rust, welcome to <:discohook:736648398081622016> **Discohook**! The easiest way to personalise your Discord server.\n\nThere's more info below, but you don't have to read it. If you're ready press **Clear All** in the top of the editor to get started.\n\nDiscohook has a [support server](https://discohook.app/discord), if you need help feel free to join in and ask questions, suggest features, or just chat with the community.\n\nWe also have [complementary bot](https://discohook.app/bot) that may help out, featuring reaction roles and other utilities.\n_ _"
//     });

//     let json_to_send = serde_json::to_string(&json_to_send).unwrap();

//     let client = Client::new();
//     let res = client
//         .post("https://discord.com/api/webhooks/1369370183541461072/tuO93OJvdUsDzbHMBwHtQex11ijpfYLZMOvMov84eUPH5or3ziU03aOUmTZkfuibdhUp")
//         .header("Content-Type", "application/json")
//         .body(json_to_send)
//         .send()
//         .await;
//     match res {
//         Ok(response) => println!("Success: {:#?}", response),
//         Err(e) => eprintln!("Error sending request: {}", e),
//     }
//     HttpResponse::Ok().body("Hello there!\nGeneral Kenobi")
// }

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
