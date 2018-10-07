use crate::database::CN;
use crate::database::{execute, query};
use crate::error::ErrorCode;
use crate::model::task::Task;
use crate::Result;
use postgres::rows::Rows;
use std::sync::Arc;

const INSERT: &str = "
insert into task(
  user_id, text, note, category,
  schedule_time, schedule_interval_value,
  schedule_interval_type, completed)
values ($1, $2, $3, $4, $5, $6, $7, $8) returning id";

const BY_USER_ID: &str = "
select
  id, text, note, category,
  schedule_time, schedule_interval_value,
  schedule_interval_type, completed
from task where user_id = $1";

const UPDATE_TASK: &str = "
update task set
  text = $3,
  note = $4,
  category = $5,
  schedule_time = $6,
  schedule_interval_value = $7,
  schedule_interval_type = $8,
  completed = $9
where user_id = $1 and id = $2";

const DELETE_TASK: &str = "
delete from task where
  user_id = $1 and 
  id = $2";

pub fn create(cn: CN, task: Arc<Task>) -> Result<i64> {
    query(
        &cn,
        INSERT,
        &[
            &task.user_id,
            &task.text,
            &task.note,
            &task.category,
            &task.schedule_time,
            &task.schedule_interval_value,
            &task.schedule_interval_type,
            &task.completed,
        ],
    )
    .and_then(|rows| match rows.len() {
        1 => Ok(rows.get(0).get(0)),
        _ => ErrorCode::DatabaseError.default().err(),
    })
}

pub fn get_by_user_id(cn: CN, user_id: i64) -> Result<Vec<Task>> {
    query(&cn, BY_USER_ID, &[&user_id]).map(|rows: Rows| {
        rows.iter()
            .map(|row| Task {
                user_id,
                id: row.get(0),
                text: row.get(1),
                note: row.get(2),
                category: row.get(3),
                schedule_time: row.get(4),
                schedule_interval_value: row.get(5),
                schedule_interval_type: row.get(6),
                completed: row.get(7),
            })
            .collect()
    })
}

pub fn update(cn: CN, task: &Task) -> Result<()> {
    execute(
        &cn,
        UPDATE_TASK,
        &[
            &task.user_id,
            &task.id,
            &task.text,
            &task.note,
            &task.category,
            &task.schedule_time,
            &task.schedule_interval_value,
            &task.schedule_interval_type,
            &task.completed,
        ],
    )
    .map(|_| ())
}

pub fn delete(cn: CN, user_id: i64, task_id: i64) -> Result<()> {
    execute(&cn, DELETE_TASK, &[&user_id, &task_id]).map(|_| ())
}
