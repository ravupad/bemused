use crate::database::task;
use crate::model::task::Task;
use crate::server::Server;

pub async fn create_task(server: Server, user_id: i64, mut task: Task) -> crate::Result<i64> {
    task.user_id = user_id;
    server
        .database
        .run(move |pool| task::create(pool, task))
        .await
}

pub fn get_task(server: Server, user_id: i64) -> crate::Result<Vec<Task>> {
    task::get_by_user_id(&server.pool, user_id)
}

pub fn update_task(server: Server, user_id: i64, mut task: Task) -> crate::Result<()> {
    task.user_id = user_id;
    task::update(&server.pool, task)
}

pub fn delete_task(server: Server, user_id: i64, task_id: i64) -> crate::Result<()> {
    task::delete(&server.pool, user_id, task_id)
}
