use crate::config::pdf_extractor::{extract_text_from_pdf, match_score};
use crate::models::application_model::Application;
use crate::repository::job_repo::JobRepo;
use crate::{config::cloudinary::CloudinaryConfig, repository::application_repo::ApplicationRepo};
use actix_multipart::Multipart;
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse,
};
use futures_util::StreamExt as _;
use log::{error, info};
use serde_json::{from_slice, json};
use tokio::{fs::File, io::AsyncWriteExt};

#[post("/applications")]
pub async fn create_application(
    db: Data<ApplicationRepo>,
    job_db: Data<JobRepo>,
    mut payload: Multipart,
) -> HttpResponse {
    let mut app_data: Option<Application> = None;
    let mut image_path: Option<String> = None;

    let mut data_bytes = web::BytesMut::new();

    // Iterate over multipart fields
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(_) => return HttpResponse::BadRequest().body("Invalid multipart data"),
        };

        // Process the field based on its name
        match field.name() {
            Some("data") => {
                // Load JSON data from the "data" part
                while let Some(chunk) = field.next().await {
                    data_bytes.extend_from_slice(&chunk.unwrap());
                }

                // Debugging output
                info!(
                    "Received application data: {:?}",
                    String::from_utf8_lossy(&data_bytes)
                );

                app_data = from_slice::<Application>(&data_bytes).ok();
                // Additional debug information

                if app_data.is_none() {
                    info!(
                        "Failed to parse job data from JSON. Raw input: {:?}",
                        app_data
                    );
                    // info!("Failed to parse job data from JSON.");
                }
            }
            Some("image") => {
                // Load image file from the "image" part
                let content_disposition = field
                    .content_disposition()
                    .expect("Missing content disposition");
                let filename = content_disposition
                    .get_filename()
                    .expect("Missing filename");
                let filepath = format!("/tmp/{}", sanitize_filename::sanitize(filename));

                let mut f = File::create(&filepath).await.unwrap();
                while let Some(chunk) = field.next().await {
                    f.write_all(&chunk.unwrap()).await.unwrap();
                }
                image_path = Some(filepath);
            }
            _ => {}
        }
    }

    let app_data = match app_data {
        Some(data) => data,
        None => {
            return HttpResponse::BadRequest().body("Missing application data");
        }
    };

    let image_path = match image_path {
        Some(path) => path,
        None => return HttpResponse::BadRequest().body("Missing image file"),
    };

    let pdf_string = extract_text_from_pdf(&image_path)
        .unwrap_or_else(|e| format!("Failed to extract text: {}", e));
    // println!("{}", pdf_string);

    let tags = match job_db.fetch_job_tags(&app_data.job_id).await {
        Ok(Some(tags)) => Ok(Some(tags)),
        Ok(None) => Ok(None),
        Err(e) => Err(format!("Error fetching job tags: {}", e)),
    };

    let score = match tags {
        Ok(Some(tags)) => match_score(&pdf_string, Some(tags)), // Pass the tags as Option<Vec<String>>
        Ok(None) => 0.0,                                        // No tags available, return 0 score
        Err(_) => 0.0, // Handle the error case, return 0 score
    };

    print!("{}", score.to_string());

    let cloudinary_config = CloudinaryConfig::new();

    let image_url = match cloudinary_config.upload_image(&image_path).await {
        Ok(response) => {
            info!("Upload successful: {}", response);
            response
        }
        Err(e) => {
            info!("Error uploading image: {:?}", e);
            return HttpResponse::BadRequest()
                .body("Image upload failed. Please provide a valid image.");
        }
    };

    let data = Application {
        id: None,
        full_name: app_data.full_name.clone(),
        resume: Some(image_url), //pass the resume image here,
        job_id: app_data.job_id.clone(),
        email: app_data.email.clone(),
        cover_letter: app_data.cover_letter.to_owned(),
        address: app_data.address.to_owned(),
        country: app_data.country.to_owned(),
        score: Some(score),
        updated_at: None,
        created_at: None,
    };
    let application_detail = db.create_application(data).await;

    match application_detail {
        Ok(app) => {
            info!("application created successfully!");

            let response_id = app.inserted_id;

            HttpResponse::Ok().json(json!({ "id": response_id }))
        }
        Err(err) => {
            error!("{}", err);
            return HttpResponse::InternalServerError().body(err.to_string());
        }
    }
}

#[get("/applications")]
pub async fn get_all_applications(db: Data<ApplicationRepo>) -> HttpResponse {
    match db.get_all_applications().await {
        Ok(apps) => HttpResponse::Ok().json(json!({ "applications": apps })),
        Err(err) => {
            error!("Failed to retrieve applications: {}", err);
            HttpResponse::InternalServerError().body("Failed to retrieve applications")
        }
    }
}
