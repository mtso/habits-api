use log::Level;
use rocket::http::Status;
use rocket::response::{Responder, Result};
use rocket::Request;

/// Response is a custom Responder implementation that describes
/// the status and responder body at the same time.
pub struct StatusResponse<T: Responder<'static>>(pub Status, pub T);

impl<T: Responder<'static>> Responder<'static> for StatusResponse<T> {
    fn respond_to(self, r: &Request) -> Result<'static> {
        let StatusResponse(status, responder) = self;

        responder
            .respond_to(r)
            .map(move |mut response| {
                response.set_status(status);
                response
            })
            .map_err(|err| {
                log!(Level::Error, "Response Error: {}", err);
                err
            })
    }
}

#[allow(dead_code)]
impl<T: Responder<'static>> StatusResponse<T> {
    pub fn ok(responder: T) -> Self {
        StatusResponse(Status::Ok, responder)
    }

    pub fn not_found(responder: T) -> Self {
        StatusResponse(Status::NotFound, responder)
    }
}
