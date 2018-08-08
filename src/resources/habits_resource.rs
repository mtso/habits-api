use rocket::http::Status;
use rocket::State;
use rocket_contrib::{Json, JsonValue};
use std::collections::HashSet;
//use std::io::ErrorKind;

use super::request_guards::wrapped_ksuid::KsuidWrapper;
use super::status_response::StatusResponse;
use processors::HabitsProcessor;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewHabitRequest {
    user_id: String,
    timezone_offset: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UncheckHabitRequest {
    checks: HashSet<String>,
}

#[post("/", data = "<new_habit_request>")]
pub fn post_habit(
    habits_processor: State<HabitsProcessor>,
    new_habit_request: Json<NewHabitRequest>,
) -> StatusResponse<Json<JsonValue>> {
    let req = new_habit_request.into_inner();
    match habits_processor.create_habit(req.user_id, req.timezone_offset) {
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

use chrono::prelude::*;

/// Used to test local date creation with chrono.
#[get("/test/<tz>")]
pub fn get_test(tz: i32) -> StatusResponse<Json<JsonValue>> {
    let nowtz = Utc::now();
    let nowdate = FixedOffset::west(tz * 3600).ymd(nowtz.year(), nowtz.month(), nowtz.day());
    let timez = nowdate.and_hms(0, 0, 0);
    let nowdate = nowdate.and_hms(nowtz.hour(), nowtz.minute(), nowtz.second());

    let tt = nowdate.naive_utc();
    StatusResponse::ok(Json(json!({
        "timestamp": timez,
        "date": nowdate.naive_utc(),
        "test": format!("{}-{:02}-{:02}", tt.year(), tt.month(), tt.day()),
    })))
}
