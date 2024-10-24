use crate::models::job_model::Job;
use crate::repository::job_repo::JobRepo;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use log::{error, info};
use serde_json::json;

#[post("/jobs")]
pub async fn create_jobs(db: Data<JobRepo>, new_job: Json<Job>) -> HttpResponse {
    let data = Job {
        id: None,
        job_title: new_job.job_title.to_owned(),
        job_availability: new_job.job_availability.clone(),
        tags: new_job.tags.to_owned(),
        job_duration: new_job.job_duration.to_owned(),
        contract_duration: new_job.contract_duration.to_owned(),
        location: new_job.location.to_owned(),
        updated_at: None,
        created_at: None,
        category: new_job.category.clone(),
        job_type: new_job.job_type.clone(),
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
