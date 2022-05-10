// /Users/matt/external_code/BrooksYew/brooks-full-stack/backend/nodejs/express/database/taskQueries.js

use crate::database::{TaskId, TodoDB, UserId};
use crate::routes::tasks::{CreateTaskRequest, Task};
//use crate::routes::TodoAppError;
use chrono::{DateTime, Utc};

// /Users/matt/external_code/BrooksYew/brooks-full-stack/database/init.sql

impl TodoDB {
    pub async fn db_insert_task(&self, task: &CreateTaskRequest, user_id: UserId) -> Option<Task> {
        let mut new_task = Task {
            id: 0,
            priority: None,
            title: task.title.clone(),
            completed_at: None,
            description: task.description.clone(),
            deleted_at: None,
            user_id,
            is_default: false,
        };

        let con = self.pool.get().await.unwrap();
        let sql = "INSERT INTO tasks (title, description) VALUES ($1, $2) RETURNING id";
        let err = con.query(sql, &[&task.title, &task.description]).await;
        if err.is_err() {
            return None;
        }

        let query_result = err.ok().unwrap();
        println!("db_insert_task {} rows returned", query_result.len());
        if let Some(row) = query_result.first() {
            new_task.id = row.get("id");
        }
        Some(new_task)
    }

    fn get_all_tasks(user_id: UserId) {
        todo!();
    }

    pub async fn db_get_task(&self, user_id: UserId, task_id: TaskId) -> Task {
        Task {
            id: 0,
            priority: None,
            title: "what".to_string(),
            completed_at: None,
            description: "task description".to_string(),
            deleted_at: None,
            user_id: 123,
            is_default: false,
        }
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
}
