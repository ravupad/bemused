use crate::database::user;
use crate::error::ErrorCode;
use crate::server::Server;
use crate::Result;
use http::header::HeaderMap;
use http::header::HeaderValue;
use slog::{info, Logger};
use std::sync::Arc;
use uuid::Uuid;

fn get_session(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .map(|a| a.to_owned())
        .ok_or_else(|| ErrorCode::NotAuthenticated.default())
}

pub async fn add_user(server: &Server, username: String, password: String) -> Result<()> {
    let result = server
        .database
        .run(move |pool| user::add_user(pool, &username, &password))
        .await
        .map(|_| ());
    result
}

pub fn create_session(server: &Server, username: String, password: String) -> Result<String> {
    let uuid = Arc::new(Uuid::new_v4().to_string());
    let uuid1 = uuid.clone();
    let uid = user::match_password(&server.pool, &username, &password);
    let r = uid.map(move |user_id| server.sled.session.save(&uuid, user_id));
    r.map(move |_| uuid1.to_string())
}

pub fn from_session_id(server: &Server, session_id: String) -> Result<i64> {
    Ok(session_id).and_then(move |sid| match server.sled.session.get(&sid) {
        Ok(Some(user_id)) => Ok(user_id),
        Ok(None) => ErrorCode::NotAuthenticated.default().err(),
        Err(err) => Err(err),
    })
}

pub fn from_header(
    logger: &Logger,
    server: &Server,
    headers: &HeaderMap<HeaderValue>,
) -> Result<i64> {
    let authorization = get_session(headers)?;
    info!(logger, "Authorization: {}", &authorization);
    let user_id = from_session_id(server, authorization)?;
    info!(logger, "UserId: {}", user_id);
    Ok(user_id)
}

pub fn remove_session(server: &Server, session_id: String) -> crate::Result<()> {
    server.sled.session.del(&session_id)
}
