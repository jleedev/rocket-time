#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::time::SystemTime;

mod api_response;
mod cors;

use api_response::ApiResponse;
use cors::Cors;

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
