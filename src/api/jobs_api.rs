use crate::config::cloudinary::CloudinaryConfig;
use crate::models::job_model::Job;
use crate::repository::job_repo::JobRepo;
use actix_multipart::Multipart;
use actix_web::{
    post,
    web::{self, Data},
    HttpResponse,
};
use futures_util::StreamExt as _;
use log::{error, info};
use serde_json::{from_slice, json};
use tokio::{fs::File, io::AsyncWriteExt};

#[post("/jobs")]
pub async fn create_jobs(
    db: Data<JobRepo>,
    // new_job: Json<Job>,
    mut payload: Multipart,
) -> HttpResponse {
    let mut job_data: Option<Job> = None;
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
                    "Received job data: {:?}",
                    String::from_utf8_lossy(&data_bytes)
                );

                job_data = from_slice::<Job>(&data_bytes).ok();
                // Additional debug information

                if job_data.is_none() {
                    info!("Failed to parse job data from JSON.");
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

    let job_data = match job_data {
        Some(data) => data,
        None => {
            return HttpResponse::BadRequest().body("Missing job data");
        }
    };

    let image_path = match image_path {
        Some(path) => path,
        None => return HttpResponse::BadRequest().body("Missing image file"),
    };

    // let image_path = "path/to/your/image.png";

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

    let data = Job {
        id: None,
        job_title: job_data.job_title.clone(),
        job_image: Some(image_url), //pass the image here,
        job_description: job_data.job_description.clone(),
        job_availability: job_data.job_availability.clone(),
        tags: job_data.tags.to_owned(),
        job_duration: job_data.job_duration.to_owned(),
        contract_duration: job_data.contract_duration.to_owned(),
        location: job_data.location.to_owned(),
        updated_at: None,
        created_at: None,
        category: job_data.category.clone(),
        job_type: job_data.job_type.clone(),
    };
    let job_detail = db.create_job(data).await;

    match job_detail {
        Ok(job) => {
            info!("job created successfully!");

            let response_id = job.inserted_id;

            HttpResponse::Ok().json(json!({ "id": response_id }))
        }
        Err(err) => {
            error!("{}", err);
            return HttpResponse::InternalServerError().body(err.to_string());
        }
    }
}
