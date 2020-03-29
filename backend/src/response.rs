use crate::Error;
use hyper::{Body, Response as HResponse, StatusCode};

pub fn response_from_json(json: impl serde::Serialize) -> HResponse<Body> {
    HResponse::builder()
        .header("Content-Type", "application/json")
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&json).map_err(Error::from).unwrap()))
        .unwrap()
}

pub fn response_from_error(err: Error) -> HResponse<Body> {
    hyper::Response::builder()
        .header("Content-Type", "application/json")
        .status(err.error_code.status_code())
        .body(hyper::Body::from(serde_json::to_string(&err).unwrap()))
        .unwrap()
}

pub fn response_from_result(result: crate::Result<HResponse<Body>>) -> HResponse<Body> {
    match result {
        Ok(response) => response,
        Err(e) => response_from_error(e),
    }
}

pub fn response_from_void(_: ()) -> HResponse<Body> {
    HResponse::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap()
}

