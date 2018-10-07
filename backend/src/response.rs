use crate::Future;
use hyper::{Body, Response as HResponse, StatusCode};

pub type FR = Future<crate::response::Response>;

pub enum Response {
    Void,
    Json(String),
    Number(i64),
}

impl Response {
    pub fn into_response(self) -> HResponse<Body> {
        match self {
            Response::Void => HResponse::builder()
                .status(StatusCode::NO_CONTENT)
                .body(Body::empty())
                .unwrap(),
            Response::Json(string) => HResponse::builder()
                .header("Content-Type", "application/json")
                .status(StatusCode::OK)
                .body(Body::from(string))
                .unwrap(),
            Response::Number(number) => HResponse::builder()
                .status(StatusCode::OK)
                .body(Body::from(number.to_string()))
                .unwrap(),
        }
    }
}

impl From<()> for Response {
    fn from(_: ()) -> Response {
        Response::Void
    }
}

impl From<String> for Response {
    fn from(string: String) -> Response {
        Response::Json(string)
    }
}

impl From<i64> for Response {
    fn from(num: i64) -> Response {
        Response::Number(num)
    }
}
