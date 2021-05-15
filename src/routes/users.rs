use rocket::*;
use rocket_contrib::json::Json;
use std::sync::Mutex;

use crate::data::state::QueueState;
use crate::models::requests::CreateJobRequest;
use crate::models::database::DatasetIngestionJob;
use crate::models::responses::CreateJobSuccessResponse;

// use crate::data::database::Conn;
// use crate::data::mongodb_connection::MongoDBConn;
use crate::data::state::Jobs;

// const DATABASE: &str = "kausa_task";
// const COLLECTION: &str = "jobs";

//////////////// Requirement #1 ////////////////

#[post("/create-job", format = "json", data = "<job_request>")]
pub fn create_job(q_state: State<QueueState>, j_state: State<Jobs>, job_request: Json<CreateJobRequest>) -> Json<CreateJobSuccessResponse> {
    // let job_request_struct: CreateJobRequest = (*job_request).clone();
    let job = Json(DatasetIngestionJob::from_job_request((*job_request).clone()));
    // Save Job to queue
    let urg_map = q_state.urgent_queues.lock().unwrap();
    let urg_map_q = &*urg_map;
    let nor_map = q_state.normal_queues.lock().unwrap();
    let nor_map_q = &*nor_map;
    if job.urgency {
        // state.urgent_queues.get_mut(&(job.protocol)).unwrap().push_back(job.clone())
        // mapQ.get_mut(&(job.protocol)).unwrap().lock().unwrap().push_back(job.clone())
        urg_map_q[&(job.data_source_protocol)].lock().unwrap().push_back(job.clone())
    } else {
        nor_map_q[&(job.data_source_protocol)].lock().unwrap().push_back(job.clone())
    }
    // Save Job to database
    let mut job_collection_map = j_state.records.lock().unwrap();
    let res = job.job_id.to_string();
    job_collection_map.insert(res.to_string(), Mutex::new(job.clone()));
    // Return job id
    Json(CreateJobSuccessResponse::from_job(&job))
}

//////////////// Requirement #4 ////////////////
/// 
#[get("/get-status/<job_id>")]
pub fn get_status(state: State<Jobs>, job_id: String) -> String {
    let job_collection_map = state.records.lock().unwrap();
    let job = job_collection_map[&job_id].lock().unwrap();
    
    format!("The status of job with ID {} is: {}", job.job_id, job.status)
}

///// Some Testing stuff

#[post("/random-job")]
pub fn random_db_job (state: State<Jobs>) -> String {
    let job = DatasetIngestionJob::random_job();
    let mut job_collection_map = state.records.lock().unwrap();
    let res = job.job_id.to_string();
    job_collection_map.insert(res.to_string(), Mutex::new(job));
    res.to_string()
}

#[get("/db-job/<id>")]
pub fn get_db_job (state: State<Jobs>, id: String) -> String {
    let job_collection_map = state.records.lock().unwrap();
    let res = job_collection_map[&id].lock().unwrap().job_id.to_string();
    res
}

#[get("/create-job")]
pub fn validate_creation(state: State<QueueState>) -> String {
    let map = state.urgent_queues.lock().unwrap();
    let map_q = &*map;
    let odbc_q = map_q["odbc"].lock().unwrap();
    format!("{}", odbc_q.len())
}