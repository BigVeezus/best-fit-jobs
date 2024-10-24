//add the modules
mod api;
mod config;
mod db;
mod models;
mod repository;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use api::{
    jobs_api::create_jobs,
    user_api::{create_user, login},
};
use colog;
use db::mongo_db::MongoRepo;
use log::info;
use repository::{job_repo::JobRepo, user_repo::UserRepo};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("healthy server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    colog::init();
    info!("Listening on port 8080");

    let mongo_repo = MongoRepo::init().await;

    // Optionally, pass the `mongo_repo` database to the `UserRepo`
    let user_repo = UserRepo::new(&mongo_repo.db);
    let job_repo: JobRepo = JobRepo::new(&mongo_repo.db);

    // Pass the repository to your Actix-web app
    let user_repo_data = Data::new(user_repo);
    let job_repo_data: Data<JobRepo> = Data::new(job_repo);

    HttpServer::new(move || {
        App::new()
            .app_data(user_repo_data.clone())
            .app_data(job_repo_data.clone())
            .service(create_user)
            .service(login)
            .service(create_jobs)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
