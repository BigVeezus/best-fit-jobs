  //add the modules
    mod api; 
    mod models;
    mod repository;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use log::info;
use colog;


    #[get("/")]
    async fn hello() -> impl Responder {
        HttpResponse::Ok().json("healthy server")
    }

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        colog::init();
        info!("Listening on port 8080"); 
        HttpServer::new(|| App::new().service(hello))
            .bind(("localhost", 8080))?
            .run()
            .await
    }