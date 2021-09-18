use actix_web::*;
use err_tools::*;
use regex::Regex;
use serde_derive::*;
use std::sync::{Arc, RwLock};

mod err;
use err::*;
mod roles;

/// This is the shared mutable state of the App,
/// The Mutex allows it to be updated in a separate thread
/// (Update not implemented yet)
type AppData = Arc<RwLock<Vec<roles::SubjectItem>>>;

/// The struct to represent the Get query Data
#[derive(Deserialize)]
pub struct RoleQuery {
    name: Option<String>,
    /// Regex filter for the name
    namex: Option<String>,
    ///"yaml"/"json" (default "json")
    output: Option<String>,
    /// Comma separatad list "User,Group,ServiceAccount",
    kind: Option<String>,
    /// "alpha"/"length" (default no sort)
    sort: Option<String>,
}

/// The main handler for splitting role data
#[get("/")]
async fn role_handle(
    dt: web::Data<AppData>,
    q: web::Query<RoleQuery>,
) -> Result<HttpResponse, AnyhowResponse> {
    let qi = q.into_inner();

    // regex built once outside of filter
    let name_reg = qi
        .namex
        .as_ref()
        .map(|s| Regex::new(s))
        .transpose()
        .e_str("Could not build regex")
        //A regex error should be returned to the caller so they can fix
        .as_err_response()?;

    // The filtered list
    let mut f_list: Vec<roles::SubjectItem> = dt
        .read()
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
                // String contains check for subject kind, seems silly makes a nice future proof one liner.
                if !kind.contains(&d.kind) {
                    return false;
                }
            }
            true
        })
        .map(|i| i.clone())
        .collect();

    match qi.sort.as_ref().map(String::as_str) {
        Some("alpha") => {
            f_list.sort_by(|a, b| a.name.cmp(&b.name));
        }
        Some("length") => {
            f_list.sort_by(|a, b| a.name.len().cmp(&b.name.len()));
        }
        _ => {}
    }

    // Choose output and content type
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
            "application/json",
        ),
    };

    Ok(HttpResponse::Ok().content_type(ctype).body(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Build the roles list before starting the server.
    // To Consider : Spawn a future to update the roles semi regularly
    let s = crate::roles::get_subjects().expect("Got Good result");
    let dt = Arc::new(RwLock::new(s));

    println!("serving on 8086");

    HttpServer::new(move || App::new().data(dt.clone()).service(role_handle))
        // To Consider Environment variables/ config for where to serve to.
        .bind("127.0.0.1:8086")?
        .run()
        .await
}
