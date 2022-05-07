// /Users/matt/external_code/BrooksYew/brooks-full-stack/backend/nodejs/express/database/taskQueries.js

use crate::routes::tasks::Task;
use chrono::{DateTime, Utc};
use crate::{UserId, TaskId};

fn insert_task() {
    todo!();
}

fn get_all_tasks(user_id: UserId) {
    todo!();
}

fn get_task(user_id: UserId, task_id: TaskId) {
    todo!();
}

fn mark_completed(user_id: UserId, task_id: TaskId) {
    todo!();
}

fn mark_uncompleted(user_id: UserId, task_id: TaskId) {
    todo!();
}

fn update_completed_status(user_id: UserId, task_id: TaskId, completed_at: DateTime<Utc>) {
    todo!();
}

fn update_task(user_id: UserId, task_id: TaskId, task: Task) {
    todo!();
} 

fn soft_delete_task(user_id: UserId, task_id: TaskId) {
    todo!();
}

fn get_default_tasks() {
    todo!();
}



