#![feature(proc_macro_hygiene, decl_macro)]

mod api_response;
mod cors;

use crate::{api_response::ApiResponse, cors::Cors};

use rocket::{get, routes};
use serde_derive::Serialize;
use std::time::SystemTime;

#[derive(Serialize)]
struct Timestamp {
    seconds: u64,
    nanos: u32,
}

#[get("/time")]
fn get_time() -> ApiResponse<Timestamp> {
    let dur = SystemTime::UNIX_EPOCH.elapsed().unwrap();
    let seconds = dur.as_secs();
    let nanos = dur.subsec_nanos();
    let out = Timestamp { seconds, nanos };
    ApiResponse(out)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![get_time])
        .attach(Cors)
        .launch();
}
