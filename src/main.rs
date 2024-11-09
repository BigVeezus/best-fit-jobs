//add the modules
mod api;
mod config;
mod db;
mod middleware;
mod models;
mod repository;

use actix_web::{get, web, web::Data, App, HttpResponse, HttpServer, Responder};
use api::{
    application_api::{create_application, get_all_applications},
    jobs_api::{create_jobs, get_all_jobs},
    user_api::{create_user, login},
};
use colog;
use db::mongo_db::MongoRepo;
use log::info;
use middleware::jwt_middleware::JwtAuthMiddleware;
use repository::{application_repo::ApplicationRepo, job_repo::JobRepo, user_repo::UserRepo};

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
    let app_repo: ApplicationRepo = ApplicationRepo::new(&mongo_repo.db);

    // Pass the repository to your Actix-web app
    let user_repo_data = Data::new(user_repo);
    let job_repo_data: Data<JobRepo> = Data::new(job_repo);
    let app_repo_data: Data<ApplicationRepo> = Data::new(app_repo);

    HttpServer::new(move || {
        App::new()
            .app_data(user_repo_data.clone())
            .app_data(job_repo_data.clone())
            .app_data(app_repo_data.clone())
            .service(create_user)
            .service(create_application)
            .service(login)
            .service(
                web::scope("/auth") // Protected scope
                    .wrap(JwtAuthMiddleware) // Apply the JWT middleware here
                    .service(create_jobs), // Endpoint under JWT protection
            )
            .service(get_all_jobs)
            .service(get_all_applications)
            .service(hello) // Public endpoint
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
