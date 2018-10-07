use crate::database::task;
use crate::model::task::Task;
use crate::server::Server;
use crate::{Error, Future};
use futures::future::{Future as _, IntoFuture};
use std::sync::Arc;

pub fn create_task(server: Server, user_id: i64, mut task: Task) -> Future<i64> {
    task.user_id = user_id;
    let fut = Ok(Arc::new(task)).into_future().and_then(move |new_task| {
        server
            .database
            .run(move |cn| task::create(cn, new_task.clone()))
    });
    Box::new(fut)
}

pub fn get_task(server: Server, user_id: i64) -> Future<String> {
    let fut = server
        .database
        .run(move |cn| task::get_by_user_id(cn, user_id))
        .and_then(|tasks| serde_json::to_string(&tasks).map_err(Error::from));
    Box::new(fut)
}

pub fn update_task(server: Server, user_id: i64, mut task: Task) -> Future<()> {
    task.user_id = user_id;
    let task = Arc::new(task);
    let fut = server.database.run(move |cn| task::update(cn, &task));
    Box::new(fut)
}

pub fn delete_task(server: Server, user_id: i64, task_id: i64) -> Future<()> {
    let fut = server
        .database
        .run(move |cn| task::delete(cn, user_id, task_id));
    Box::new(fut)
}
