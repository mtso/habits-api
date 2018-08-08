use rocket::http::Status;
use rocket::State;
use rocket_contrib::{Json, JsonValue};
use std::collections::HashSet;
//use std::io::ErrorKind; // FIXME: properly handle different errors

use super::request_guards::wrapped_ksuid::KsuidWrapper;
use super::status_response::StatusResponse;
use processors::HabitsProcessor;

/// Request structs

#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct NewHabitRequest {
    user_id: String,
    timezone_offset: i32,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UncheckHabitRequest {
    checks: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct UserFilter {
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct UpdateHabitRequest {
    timezone_offset: Option<i32>,
    title: Option<String>,
}

#[post("/", data = "<new_habit_request>")]
pub fn post_habit(
    habits_processor: State<HabitsProcessor>,
    new_habit_request: Json<NewHabitRequest>,
) -> StatusResponse<Json<JsonValue>> {
    let req = new_habit_request.into_inner();
    match habits_processor.create_habit(req.user_id, req.timezone_offset, req.title) {
        Ok(habit) => StatusResponse::ok(habit.to_external()),
        Err(e) => StatusResponse(
            Status::InternalServerError,
            Json(json!({
                "error": format!("{}", e),
            })),
        ),
    }
}

#[get("/<id>")]
pub fn get_habit(
    habits_processor: State<HabitsProcessor>,
    id: KsuidWrapper,
) -> StatusResponse<Json<JsonValue>> {
    //    habits_processor
    //        .get_habit(id.unwrap())
    //        .map(|h| { StatusResponse::ok(h.to_external()) })
    //        .map_err(|e| { StatusResponse(Status::NotFound, Json(json!({ "error": format!("{}", e) }))) })
    //        .unwrap_or(StatusResponse(Status::InternalServerError, Json(json!({
    //            "error": "Something terribly wrong happened",
    //        }))))

    match habits_processor.get_habit(id.unwrap()) {
        Ok(habit) => StatusResponse(Status::Ok, habit.to_external()),
        Err(e) => StatusResponse(Status::NotFound, Json(json!({ "error": format!("{}", e) }))),
    }
}

#[get("/?<user_filter>")]
pub fn get_habits(
    habits_processor: State<HabitsProcessor>,
    user_filter: UserFilter,
) -> StatusResponse<Json<JsonValue>> {
    use serde_json::Value;

    match habits_processor.get_habits_by_userid(user_filter.user_id) {
        Ok(habits) => {
            let habits = habits
                .iter()
                .map(|h| {
                    let JsonValue(ref v) = *h.to_external();
                    v.clone()
                })
                .collect();

            StatusResponse::ok(Json(JsonValue(Value::Array(habits))))
        }
        Err(e) => StatusResponse(
            Status::InternalServerError,
            Json(json!({ "error": format!("{}", e) })),
        ),
    }
}

#[put("/<id>", data = "<update_habit_request>")]
pub fn put_habit(
    habits_processor: State<HabitsProcessor>,
    id: KsuidWrapper,
    update_habit_request: Json<UpdateHabitRequest>,
) -> StatusResponse<Json<JsonValue>> {
    let req = update_habit_request.into_inner();
    match habits_processor.update_habit(id.unwrap(), req.title, req.timezone_offset) {
        Ok(h) => StatusResponse::ok(h.to_external()),
        Err(e) => StatusResponse(Status::NotFound, Json(json!({ "error": format!("{}", e) }))),
    }
}

#[delete("/<id>")]
pub fn delete_habit(
    habits_processor: State<HabitsProcessor>,
    id: KsuidWrapper,
) -> StatusResponse<Json<JsonValue>> {
    match habits_processor.delete_habit(id.unwrap()) {
        Ok(()) => StatusResponse::ok(Json(json!({}))),
        Err(e) => StatusResponse(Status::NotFound, Json(json!({ "error": format!("{}", e) }))),
    }
}

#[post("/<id>/check")]
pub fn check_habit(
    habits_processor: State<HabitsProcessor>,
    id: KsuidWrapper,
) -> StatusResponse<Json<JsonValue>> {
    match habits_processor.check_habit(id.unwrap()) {
        Ok(habit) => StatusResponse(Status::Ok, habit.to_external()),
        Err(e) => StatusResponse(Status::NotFound, Json(json!({ "error": format!("{}", e) }))),
    }
}

#[post("/<id>/uncheck", data = "<uncheck_habit_request>")]
pub fn uncheck_habit(
    habits_processor: State<HabitsProcessor>,
    id: KsuidWrapper,
    uncheck_habit_request: Json<UncheckHabitRequest>,
) -> StatusResponse<Json<JsonValue>> {
    match habits_processor.uncheck_habit(id.unwrap(), uncheck_habit_request.into_inner().checks) {
        Ok(habit) => StatusResponse(Status::Ok, habit.to_external()),
        Err(e) => StatusResponse(Status::NotFound, Json(json!({ "error": format!("{}", e) }))),
    }
}

#[post("/<id>/reset")]
pub fn reset_habit_checks(
    habits_processor: State<HabitsProcessor>,
    id: KsuidWrapper,
) -> StatusResponse<Json<JsonValue>> {
    match habits_processor.reset_habit_checks(id.unwrap()) {
        Ok(habit) => StatusResponse(Status::Ok, habit.to_external()),
        Err(e) => StatusResponse(Status::NotFound, Json(json!({ "error": format!("{}", e) }))),
    }
}
