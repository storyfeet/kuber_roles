use actix_web::*;
use err_tools::*;
use serde_derive::*;
use std::sync::{Arc, Mutex};

mod err;
use err::*;
mod roles;

type AppData = Arc<Mutex<roles::KubeOut>>;

#[derive(Deserialize)]
pub struct RoleQuery {
    name: Option<String>,
}

#[get("/roles")]
async fn role_handle(
    dt: web::Data<AppData>,
    q: web::Query<RoleQuery>,
) -> Result<String, AnyhowResponse> {
    let qi = q.into_inner();
    let f_list: Vec<roles::RoleItem> = dt
        .lock()
        .ok()
        .e_str("could not lock data")
        .as_err_response()?
        .items
        .iter()
        .filter(|d| {
            if let Some(nq) = &qi.name {
                if !d.has_subject_name(nq) {
                    return false;
                }
            }
            true
        })
        .map(|i| i.clone())
        .collect();
    let res = format!("{:?}", f_list);
    Ok(res)
}

#[get("/")]
async fn index_form() -> &'static str {
    "Hello"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let s = crate::roles::get_roles().expect("Got Good result");
    println!("ROLES === {:?} === ROLES", s);
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
