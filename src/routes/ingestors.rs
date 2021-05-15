use rocket::*;
use rocket_contrib::json::Json;
use std::sync::Mutex;
use uuid::Uuid;
use crate::models::database::DatasetIngestionJob;
use crate::data::state::QueueState;
use crate::data::state::Jobs;
use crate::models::requests::CreateResultRequest;


//////////////// Requirement #2 ////////////////

#[get("/request-job/<protocol>")]
pub fn request_job(state: State<QueueState>, protocol: String) ->  Json<DatasetIngestionJob>{
    let map_urgent = state.urgent_queues.lock().unwrap();
    let map_normal = state.normal_queues.lock().unwrap();
    let map_urgent_q = &*map_urgent;
    let map_normal_q = &*map_normal;
    let mut target_urgent_queue = map_urgent_q[&protocol].lock().unwrap();
    let mut target_normal_queue = map_normal_q[&protocol].lock().unwrap();
    if target_urgent_queue.is_empty() {
        if target_normal_queue.is_empty() {
            /**
             * @Todo Make endpoint return enum of DataIngestionJob or Generic ErrorResponse
             */
            Json(DatasetIngestionJob::random_job())
        }
        else{
            let returned_job = target_normal_queue.get(0).unwrap().clone();
            target_normal_queue.pop_front();
            Json(returned_job) 
        }
    }
    else{
        let returned_job = target_urgent_queue.get(0).unwrap().clone();
        target_urgent_queue.pop_front();
        Json(returned_job)
    }
    //let mut odbc_q = map_q["odbc"].lock().unwrap();
    //format!("{}", odbc_q.len())
    /*let returned_job = odbc_q.get(0).unwrap().clone();
    odbc_q.pop_front();
    Json(returned_job)*/
    
}

//////////////// Requirement #3 ////////////////
#[post("/update-result", format = "json", data = "<job_result>")]
pub fn post_result(state: State<Jobs>, job_result: Json<CreateResultRequest>) -> String {
    let job_id = job_result.job_id.to_string();
    let mut job_collection_map = state.records.lock().unwrap();
    let mut struct_in_hand = job_collection_map[&job_id].lock().unwrap();
    struct_in_hand.result_id = Uuid::new_v4().to_string();
    struct_in_hand.status = job_result.status.to_string();
    struct_in_hand.dataset_location = job_result.dataset_location.to_string();
    // job_collection_map.insert(job_id, Mutex::new(struct_in_hand.clone()));
    
    format! ("Status of job with ID {} is updated", job_id.to_string())
}