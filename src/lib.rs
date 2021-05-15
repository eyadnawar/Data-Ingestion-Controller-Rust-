#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use]
use rocket::*;
// use rocket_contrib::serve::StaticFiles;
// use rocket_contrib::helmet::SpaceHelmet;

mod routes;
mod data;
mod models;

pub fn rocket_builder() -> rocket::Rocket {
    rocket::ignite()
    // .attach(SpaceHelmet::default())
    .mount("/", routes![routes::users::create_job, routes::ingestors::request_job, routes::ingestors::post_result, routes::users::get_status, routes::generators::get_dataset])
    .manage(data::state::QueueState::init_queues())
    .manage(data::state::Jobs::init_job_collection())
    // .manage(data::mongodb_connection::MongoDBConn::new())
    // .manage(data::database::init_pool())
}
