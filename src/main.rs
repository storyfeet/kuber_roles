use actix_web::*;

//mod err;
mod roles;

#[get("/")]
async fn index() -> impl Responder {
    "Hello All"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let s = crate::roles::get_roles().expect("Got Good result");
    println!("ROLES === {:?} === ROLES", s);

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8086")?
        .run()
        .await
}
