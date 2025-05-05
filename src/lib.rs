use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use std::fs;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello there!\nGeneral Kenobi")
}

// the path to get to the html response
#[get("/resend")]
// function signature, data that is passed in, return type must implement the Responder trait
pub async fn resend(req_body: String) -> impl Responder {
    // this returns a html response with a 200 code
    // this should be used for final serialization
    // possibly main functionality
    HttpResponse::Ok().body(req_body)
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
pub async fn skills_home() -> impl Responder {
    let raw_yaml: String = fs::read_to_string("./src/data_txt/skill_level.yaml").unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
