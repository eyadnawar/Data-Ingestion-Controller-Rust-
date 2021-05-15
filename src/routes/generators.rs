use rocket::*;
use crate::data::state::Jobs;


//////////////// Requirement #5 ////////////////
/// 
#[get("/get-dataset/<job_id>")]
pub fn get_dataset(state: State<Jobs>, job_id: String) -> String {
    let job_collection_map = state.records.lock().unwrap();
    let job = job_collection_map[&job_id].lock().unwrap();
    
    format!("The location of job with ID {} is: {}", job.job_id, job.dataset_location)
}