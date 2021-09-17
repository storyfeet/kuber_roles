use actix_web::*;
use err_tools::*;
use regex::Regex;
use serde_derive::*;
use std::sync::{Arc, Mutex};

mod err;
use err::*;
mod roles;

type AppData = Arc<Mutex<Vec<roles::SubjectItem>>>;

#[derive(Deserialize)]
pub struct RoleQuery {
    name: Option<String>,
    namex: Option<String>,
    output: Option<String>,
    kind: Option<String>,
}

#[get("/roles")]
async fn role_handle(
    dt: web::Data<AppData>,
    q: web::Query<RoleQuery>,
) -> Result<HttpResponse, AnyhowResponse> {
    let qi = q.into_inner();
    let name_reg = qi
        .namex
        .as_ref()
        .map(|s| Regex::new(s))
        .transpose()
        .e_str("Could not build regex")
        .as_err_response()?;

    let f_list: Vec<roles::SubjectItem> = dt
        .lock()
        .ok()
        .e_str("could not lock data")
        .as_err_response()?
        .iter()
        .filter(|d| {
            if let Some(nq) = &qi.name {
                if !d.name.contains(nq) {
                    return false;
                }
            }
            if let Some(rg) = &name_reg {
                if !rg.is_match(&d.name) {
                    return false;
                }
            }
            if let Some(kind) = &qi.kind {
                if !kind.contains(&d.kind) {
                    return false;
                }
            }
            true
        })
        .map(|i| i.clone())
        .collect();

    let (res, ctype) = match qi.output.as_ref().map(String::as_str) {
        Some("yaml") => (
            serde_yaml::to_string(&f_list)
                .e_str("could not Yamlize")
                .as_err_response()?,
            "text/yaml",
        ),
        _ => (
            serde_json::to_string(&f_list)
                .e_str("Could not Jsonise")
                .as_err_response()?,
            "text/json",
        ),
    };

    Ok(HttpResponse::Ok().content_type(ctype).body(res))
}

#[get("/")]
async fn index_form() -> &'static str {
    "Hello"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let s = crate::roles::get_subjects().expect("Got Good result");
    println!("Subjects === {:?} === Subjects", s);
    let dt = Arc::new(Mutex::new(s));

    HttpServer::new(move || {
        App::new()
            .data(dt.clone())
            .service(role_handle)
            .service(index_form)
    })
    .bind("127.0.0.1:8086")?
    .run()
    .await
}
