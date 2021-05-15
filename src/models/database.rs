use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::requests::CreateJobRequest;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatasetIngestionJob {
    #[serde(rename = "_id")]
    pub job_id: String,
    pub user_id: String,
    pub urgency: bool,
    pub data_source_protocol: String,
    pub text_query: String,
    pub url: String,
    
    pub result_id: String,
    pub status: String,
    pub dataset_location: String
}
impl DatasetIngestionJob {
    pub fn new(user_id: String, urgency: bool, data_source_protocol: String, text_query: String, url: String) -> Self {
        DatasetIngestionJob {
            job_id: Uuid::new_v4().to_string(),
            user_id,
            urgency,
            data_source_protocol,
            text_query,
            url,
            result_id: "".to_string(),
            status: "pending".to_string(),
            dataset_location: "".to_string()

        }    
    }
    pub fn from_job_request(job_request: CreateJobRequest) -> Self {
        DatasetIngestionJob::new(job_request.user_id, job_request.urgency, job_request.data_source_protocol, job_request.text_query, job_request.url)
    }
    pub fn random_job() -> Self {
        DatasetIngestionJob::new(Uuid::new_v4().to_string(), true, "jdbc".to_string(), "random".to_string(), "random tany".to_string())
    }
}