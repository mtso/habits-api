use rocket::http::Status;
use rocket::State;
use rocket_contrib::{Json, JsonValue};

use super::status_response::StatusResponse;
use processors::HabitsProcessor;

#[get("/healthcheck")]
pub fn healthcheck(_token_processor: State<HabitsProcessor>) -> StatusResponse<Json<JsonValue>> {
    StatusResponse(
        Status::Accepted,
        Json(json!({ "message": "Not implemented" })),
    )
}
