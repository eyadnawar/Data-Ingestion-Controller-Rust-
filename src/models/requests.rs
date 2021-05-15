use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateJobRequest {
    pub user_id: String,
    pub urgency: bool,
    pub data_source_protocol: String,
    pub text_query: String,
    pub url: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateResultRequest {
    pub job_id: String,
    pub status: String,
    pub dataset_location: String 
}