use mongodb::Client;
use std::sync::Mutex;

pub struct MongoDBConn {
    pub client: Mutex<Client>
}


impl MongoDBConn {
    pub async fn new() -> Self {
        // Parse a connection string into an options struct.
        let clientt = Client::with_uri_str("mongodb://localhost:27017/").await.unwrap();
        MongoDBConn {
            client: Mutex::new(clientt)
        }
    }
}


/* Alternatively
let options = ClientOptions::builder()
                  .hosts(vec![
                      StreamAddress {
                          hostname: "localhost".into(),
                          port: Some(27017),
                      }
                  ])
                  .build();

let client = Client::with_options(options)?;

const db: mongodb::Database = client.database("kausa_task");

const job_coll: mongodb::collection = db.collection("job");
*/
