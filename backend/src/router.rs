use crate::error::{Error, ErrorCode};
use crate::request::Request;
use crate::response::*;
use crate::server::Server;
use crate::service::{article, task, user};
use crate::Result;
use http::Method;
use slog::Logger;

fn h1<'a, T>(server: &Server, rc: &'a Request) -> Result<(i64, T)>
where
    T: serde::de::Deserialize<'a>,
{
    user::from_header(server, &rc.headers).and_then(|user_id| {
        serde_json::from_slice(&rc.body)
            .map_err(Error::from)
            .map(|body| (user_id, body))
    })
}

pub async fn router(log: Logger, server: Server, rc: Request) -> Result<hyper::Response<hyper::Body>> {
    match (rc.method.clone(), &rc.path()[..]) {
        // User
        (Method::POST, ["user", username, password]) => {
            user::add_user(log, server, username.to_string(), password.to_string())
                .await
                .map(response_from_void)
        }
        (Method::PUT, ["user", username, password]) => {
            user::create_session(server, username.to_string(), password.to_string())
                .map(response_from_json)
        }
        (Method::GET, ["user", session_id]) => {
            user::from_session_id(&server, session_id.to_string())
                .map(response_from_json)
        }
        (Method::DELETE, ["user", session_id]) => {
            user::remove_session(server, session_id.to_string())
                .map(response_from_void)
        }
        // Task
        (Method::POST, ["task"]) => {
            let (user_id, task) = h1(&server, &rc)?;
            task::create_task(server, user_id, task).await
                .map(response_from_json)
        }
        (Method::GET, ["task"]) => {
            user::from_header(&server, &rc.headers)
                .and_then(|user_id| task::get_task(server, user_id))
                .map(response_from_json)
        }
        (Method::PUT, ["task"]) => {
            let (user_id,task) = h1(&server, &rc)?;
            task::update_task(server, user_id, task)
                .map(response_from_json)
        }
        (Method::DELETE, ["task", task_id]) => {
            let task_id = task_id.parse().map_err(Error::from)?;
            user::from_header(&server, &rc.headers)
                .and_then(|user_id| task::delete_task(server, user_id, task_id))
                .map(response_from_json)
        }
        // Article
        (Method::POST, ["article"]) => {
            let (user_id, article) = h1(&server, &rc)?;
            article::create(server, user_id, article)
                .map(response_from_json)
        }
        (Method::GET, ["article", article_id]) => {
            let article_id = article_id.to_string();
            let user_id = user::from_header(&server, &rc.headers)?;
            article::get(server, user_id, article_id)
                .map_err(Error::from)
                .map(response_from_json)
        }
        (Method::GET, ["article"]) => {
            let user_id = user::from_header(&server, &rc.headers)?;
            article::list(server, user_id)
                .map_err(Error::from)
                .map(response_from_json)
        }
        (Method::PUT, ["article"]) => {
            let (user_id, article) = h1(&server, &rc)?;
            article::update(server, user_id, article)
                .map(response_from_json)
        }
        (Method::DELETE, ["article", article_id]) => {
            let article_id = article_id.to_string();
            let user_id = user::from_header(&server, &rc.headers)?;
            article::delete(server, user_id, article_id)
                .map(response_from_json)
        }
        _ => ErrorCode::NotFound.default().err(),
    }
}
