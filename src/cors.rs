use rocket::fairing::*;
use rocket::http::hyper::header::*;
use rocket::{Request, Response};

/// Unconditionally allow origin * on all responses
pub struct Cors;

impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS Header",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(AccessControlAllowOrigin::Any);
    }
}
