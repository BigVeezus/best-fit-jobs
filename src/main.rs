  //add the modules
    mod api; 
    mod models;
    mod repository;
    mod db;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web::Data};
use api::user_api::create_user;
use db::mongo_db::MongoRepo;
use repository::user_repo::UserRepo;
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

        
        let mongo_repo = MongoRepo::init().await;

        // Optionally, pass the `mongo_repo` database to the `UserRepo`
        let user_repo = UserRepo::new(&mongo_repo.db);
    
        // Pass the repository to your Actix-web app
        let user_repo_data = Data::new(user_repo);

            HttpServer::new(move || {
                App::new()
                    .app_data(user_repo_data.clone())
                    .service(create_user)
            })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
    }