use rocket::response::{content, Responder, Result};
use rocket::Request;
use rocket_contrib::json::Json;
use rocket_contrib::msgpack::MsgPack;
use serde::Serialize;

/// Produce multiple formats from a single route.
///
/// - If HTML is requested, the response contains pretty-printed JSON.
/// - MsgPack is produced if requested.
/// - Otherwise, the default is compact JSON.
pub struct ApiResponse<T: Serialize>(pub T);

impl<'r, T: Serialize> Responder<'r> for ApiResponse<T> {
    fn respond_to(self, req: &Request) -> Result<'r> {
        match req.format() {
            Some(f) if f.is_html() => serde_json::to_string_pretty(&self.0)
                .map(content::Json)
                .respond_to(req),
            Some(f) if f.is_msgpack() => MsgPack(self.0).respond_to(req),
            _ => Json(self.0).respond_to(req),
        }
    }
}
