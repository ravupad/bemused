use crate::error::{Error, ErrorCode};
use crate::response::*;
use crate::server::get_body;
use crate::server::get_path;
use crate::server::Server;
use crate::service::{article, task, user};
use crate::Result;
use http::Method;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use slog::Logger;
use slog::info;

async fn h1<T>(logger: &Logger, server: &Server, request: Request<Body>) -> Result<(i64, T)>
where
    T: serde::de::DeserializeOwned,
{
    let user_id: i64 = user::from_header(logger, server, &request.headers())?;
    let body: Vec<u8> = get_body(request.into_body()).await;
    let t: T = serde_json::from_slice(&body).map_err(Error::from)?;
    Ok((user_id, t))
}

pub async fn router(
    logger: Logger,
    server: Server,
    request: Request<Body>,
) -> Result<Response<Body>> {
    info!(logger, "Url: {}",&request.uri().path());
    match &get_path(&request.uri().path(), 0, 1)[..] {
        ["user"] => user_router(logger, server, request).await,
        ["task"] => task_router(logger, server, request).await,
        ["article"] => article_router(logger, server, request).await,
        _ => ErrorCode::NotFound.message("router not found").err(),
    }
}

async fn user_router(_: Logger, server: Server, request: Request<Body>) -> Result<Response<Body>> {
    match (
        request.method().clone(),
        &get_path(&request.uri().path(), 1, 2)[..],
    ) {
        (Method::POST, [username, password]) => {
            user::add_user(&server, username.to_string(), password.to_string())
                .await
                .map(response_from_void)
        }
        (Method::PUT, [username, password]) => {
            user::create_session(&server, username.to_string(), password.to_string())
                .map(response_from_json)
        }
        (Method::GET, [session_id]) => {
            user::from_session_id(&server, session_id.to_string()).map(response_from_json)
        }
        (Method::DELETE, [session_id]) => {
            user::remove_session(&server, session_id.to_string()).map(response_from_void)
        }
        _ => ErrorCode::NotFound
            .message("user_router function not found")
            .err(),
    }
}

async fn task_router(
    logger: Logger,
    server: Server,
    request: Request<Body>,
) -> Result<Response<Body>> {
    match (
        request.method().clone(),
        &get_path(&request.uri().path(), 1, 2)[..],
    ) {
        (Method::POST, []) => {
            let (user_id, task) = h1(&logger, &server, request).await?;
            task::create_task(server, user_id, task)
                .await
                .map(response_from_json)
        }
        (Method::GET, []) => user::from_header(&logger, &server, request.headers())
            .and_then(|user_id| task::get_task(server, user_id))
            .map(response_from_json),
        (Method::PUT, []) => {
            let (user_id, task) = h1(&logger, &server, request).await?;
            task::update_task(server, user_id, task).map(response_from_json)
        }
        (Method::DELETE, [task_id]) => {
            let task_id = task_id.parse().map_err(Error::from)?;
            user::from_header(&logger, &server, &request.headers())
                .and_then(|user_id| task::delete_task(server, user_id, task_id))
                .map(response_from_json)
        }
        _ => ErrorCode::NotFound
            .message("task_router function not found")
            .err(),
    }
}

async fn article_router(
    logger: Logger,
    server: Server,
    request: Request<Body>,
) -> Result<Response<Body>> {
    match (
        request.method().clone(),
        &get_path(&request.uri().path(), 1, 2)[..],
    ) {
        (Method::POST, []) => {
            let (user_id, article) = h1(&logger, &server, request).await?;
            article::create(server, user_id, article).map(response_from_json)
        }
        (Method::GET, [article_id]) => {
            let article_id = article_id.to_string();
            let user_id = user::from_header(&logger, &server, &request.headers())?;
            article::get(server, user_id, article_id)
                .map_err(Error::from)
                .map(response_from_json)
        }
        (Method::GET, []) => {
            let user_id = user::from_header(&logger, &server, &request.headers())?;
            article::list(server, user_id)
                .map_err(Error::from)
                .map(response_from_json)
        }
        (Method::PUT, []) => {
            let (user_id, article) = h1(&logger, &server, request).await?;
            article::update(server, user_id, article).map(response_from_json)
        }
        (Method::DELETE, [article_id]) => {
            let article_id = article_id.to_string();
            let user_id = user::from_header(&logger, &server, &request.headers())?;
            article::delete(server, user_id, article_id).map(response_from_json)
        }
        _ => ErrorCode::NotFound
            .message("article_router function not found")
            .err(),
    }
}
