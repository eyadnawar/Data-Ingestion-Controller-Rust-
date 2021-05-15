use serde::{Deserialize, Serialize};

use super::database::DatasetIngestionJob;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateJobSuccessResponse {
    pub status: String,
    pub message: String,
    pub job_id: String,
}

impl CreateJobSuccessResponse {
    pub fn from_job(job: &DatasetIngestionJob)-> Self {
        CreateJobSuccessResponse{
            status: "200".to_string(),
            message: "Job Registered Successfully".to_string(),
            job_id: job.job_id.to_string()
        }
    }
}



/*
#[derive(Serialize, Deserialize)]
pub struct Response {
    status: String,
    message: String,
}
impl Response {
    fn ok(msg: &str) -> Self {
        Response {
            status: "Success".to_string(),
            message: msg.to_string(),
        }
    }
    fn err(msg: &str) -> Self {
        Response {
            status: "Error".to_string(),
            message: msg.to_string(),
        }
    }
}
*/