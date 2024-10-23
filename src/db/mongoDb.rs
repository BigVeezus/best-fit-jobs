use std::env;
    extern crate dotenv;
    use dotenv::dotenv;
    use log::{error, warn, info, debug, trace};


    use mongodb::{
        bson::extjson::de::Error,
        results::InsertOneResult,
        Client, Collection,
    };
    use crate::models::user_model::User;

    pub struct MongoRepo {
        col: Collection<User>,
    }

   

    impl MongoRepo {
        pub async fn init() -> Self {
            dotenv().ok();
            let uri = match env::var("MONGO_URI") {
                Ok(v) => v.to_string(),
                Err(_) => error!("info message"),
            };
            let client = Client::with_uri_str(uri).unwrap();
            info!("mongodb connected");
            let db = client.database("rustDB");
            MongoRepo { col }
        }
    }