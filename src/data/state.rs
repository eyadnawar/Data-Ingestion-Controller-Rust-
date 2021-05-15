use std::collections::VecDeque;
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

use crate::models::database::DatasetIngestionJob;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueueState {
    pub urgent_queues: Mutex<HashMap<String, Mutex<VecDeque<DatasetIngestionJob>>>>,
    pub normal_queues: Mutex<HashMap<String, Mutex<VecDeque<DatasetIngestionJob>>>> 
}

impl QueueState {
    pub fn init_queues () -> Self {
        let mut urgent = HashMap::new();
        let empty_vector: VecDeque<DatasetIngestionJob> = VecDeque::new();
        urgent.insert("jdbc".to_string(), Mutex::new(empty_vector.clone()));
        urgent.insert("odbc".to_string(), Mutex::new(empty_vector.clone()));
        urgent.insert("s3".to_string(), Mutex::new(empty_vector.clone()));
        urgent.insert("looker".to_string(), Mutex::new(empty_vector.clone()));
        let mut normal = HashMap::new();
        normal.insert("jdbc".to_string(), Mutex::new(empty_vector.clone()));
        normal.insert("odbc".to_string(), Mutex::new(empty_vector.clone()));
        normal.insert("s3".to_string(), Mutex::new(empty_vector.clone()));
        normal.insert("looker".to_string(), Mutex::new(empty_vector.clone()));


        QueueState {
            urgent_queues: Mutex::new(urgent),
            normal_queues: Mutex::new(normal)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Jobs {
    pub records: Mutex<HashMap<String, Mutex<DatasetIngestionJob>>>
}

impl Jobs {
    pub fn init_job_collection () -> Self {
        Jobs {
            records: Mutex::new(HashMap::new())
        }
    }
}