#![feature(proc_macro_hygiene, decl_macro)]

mod api_response;

use crate::api_response::ApiResponse;

use rocket::{fairing::AdHoc, get, routes};
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

/// Unconditionally allow origin * on all responses
fn cors() -> AdHoc {
    use rocket::http::hyper::header::*;
    AdHoc::on_response("CORS Header", |_req, resp| {
        resp.set_header(AccessControlAllowOrigin::Any);
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![get_time])
        .attach(cors())
        .launch();
}
