use crate::error::{Error, ErrorCode};
use crate::request::Request;
use crate::response::{Response, FR};
use crate::server::Server;
use crate::service::{article, task, user};
use futures::future::{Future as _, IntoFuture};
use crate::Result;
use hyper::Method;
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

pub fn router(log: Logger, server: Server, rc: Request) -> FR {
    match (rc.method.clone(), &rc.path()[..]) {
        // User
        (Method::POST, ["user", username, password]) => Box::new(
            user::add_user(log, server, username.to_string(), password.to_string())
                .map(Response::from),
        ),
        (Method::PUT, ["user", username, password]) => Box::new(
            user::create_session(server, username.to_string(), password.to_string())
                .map(Response::from),
        ),
        (Method::GET, ["user", session_id]) => Box::new(
            user::from_session_id(&server, session_id.to_string())
                .into_future()
                .map(Response::from),
        ),
        (Method::DELETE, ["user", session_id]) => {
            Box::new(user::remove_session(server, session_id.to_string()).map(Response::from))
        }
        // Task
        (Method::POST, ["task"]) => Box::new(
            h1(&server, &rc)
                .into_future()
                .and_then(|(user_id, task)| task::create_task(server, user_id, task))
                .map(Response::from),
        ),
        (Method::GET, ["task"]) => Box::new(
            user::from_header(&server, &rc.headers)
                .into_future()
                .and_then(|user_id| task::get_task(server, user_id))
                .map(Response::from)
        ),
        (Method::PUT, ["task"]) => Box::new(
            h1(&server, &rc)
                .into_future()
                .and_then(|(user_id, task)| task::update_task(server, user_id, task))
                .map(Response::from)
        ),
        (Method::DELETE, ["task", task_id]) => Box::new(
             task_id.parse().map_err(Error::from)
                .and_then(|task_id| 
                    user::from_header(&server, &rc.headers)
                        .map(|user_id| (user_id, task_id))
                )
                .into_future()
                .and_then(|(user_id, task_id)| task::delete_task(server, user_id, task_id))
                .map(Response::from)
        ),
        // Article
        (Method::POST, ["article"]) => Box::new(
            h1(&server, &rc)
                .into_future()
                .and_then(|(user_id, article)| article::create(server, user_id, article))
                .map(Response::from)
        ),
        (Method::GET, ["article", article_id]) => {
            let article_id = article_id.to_string();
            Box::new(
                user::from_header(&server, &rc.headers)
                    .into_future()
                    .and_then(|user_id| article::get(server, user_id, article_id))
                    .and_then(|article| serde_json::to_string(&article).map_err(Error::from))
                    .map(Response::from)
            )
        },
        (Method::GET, ["article"]) => Box::new(
            user::from_header(&server, &rc.headers)
                .into_future()
                .and_then(|user_id| article::list(server, user_id))
                .and_then(|articles| serde_json::to_string(&articles).map_err(Error::from))
                .map(Response::from)
        ),
        (Method::PUT, ["article"]) => Box::new(
            h1(&server, &rc)
                .into_future()
                .and_then(|(user_id, article)| article::update(server, user_id, article))
                .map(Response::from)
        ),
        (Method::DELETE, ["article", article_id]) => {
            let article_id = article_id.to_string();
            Box::new(
                user::from_header(&server, &rc.headers)
                    .into_future()
                    .and_then(|user_id| article::delete(server, user_id, article_id))
                    .map(Response::from)
            )
        },
        _ => Box::new(ErrorCode::NotFound.default().err().into_future()),
    }
}
