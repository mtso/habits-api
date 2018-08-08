use rocket::Request;
use rocket_contrib::{Json, JsonValue};

#[catch(404)]
pub fn not_found(_: &Request) -> Json<JsonValue> {
    Json(json!({
        "error": "Not found",
    }))
}

#[catch(400)]
pub fn bad_request(_: &Request) -> Json<JsonValue> {
    Json(json!({
        "error": "Bad request",
    }))
}

#[catch(500)]
pub fn internal_server_error(_: &Request) -> Json<JsonValue> {
    Json(json!({
        "error": "Internal server error",
    }))
}
