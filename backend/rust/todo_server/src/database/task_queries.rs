// /Users/matt/external_code/BrooksYew/brooks-full-stack/backend/nodejs/express/database/taskQueries.js

use crate::database::{TaskId, TodoDB, UserId};
use crate::routes::tasks::{TaskInfo, CreateTaskRequest, Task};
//use crate::routes::TodoAppError;
use chrono::{DateTime, Utc};

// /Users/matt/external_code/BrooksYew/brooks-full-stack/database/init.sql

impl TodoDB {
    pub async fn db_insert_task(&self, task: &CreateTaskRequest, user_id: UserId) -> Option<TaskInfo> {

        let con = self.pool.get().await.unwrap();
        let sql = "INSERT INTO tasks (title, description, user_id) VALUES ($1, $2, $3) RETURNING id, priority, title, completed_at, description";
        let err = con.query(sql, &[&task.title, &task.description.clone(), &user_id]).await;
        if err.is_err() {
            return None;
        }
        let query_result = err.ok().unwrap();
        println!("db_insert_task {} rows returned", query_result.len());
        if let Some(row) = query_result.first() {
            return Some(TaskInfo {
                id: row.get("id"), 
                priority: row.get("priority"), 
                title: row.get("title"),
                completed_at: row.get("completed_at"),
                description: row.get("description"), 
            });
        }
        None
    }

    fn get_all_tasks(user_id: UserId) {
        todo!();
    }

    pub async fn db_get_task(&self, user_id: UserId, task_id: TaskId) -> Option<Task> {
        
        let con = self.pool.get().await.unwrap();
        let sql = "SELECT * FROM tasks WHERE user_id = $1 AND id = $2";
        let err = con.query(sql, &[&user_id, &task_id]).await;
        if err.is_err() {
            let e = err.err().unwrap();
            if let Some(db_err) = e.as_db_error() {
                println!("db_get_task error {}", db_err.message().to_string());
            } else {
                println!("where is the error");
            }
            return None;
        }
        let query_result = err.ok().unwrap();
        if let Some(row) = query_result.first() {
            return Some(Task {
                id: row.get("id"),
                priority: row.get("priority"),
                title: row.get("title"), 
                completed_at: row.get("completed_at"), 
                description: row.get("description"), 
                deleted_at: row.get("deleted_at"), 
                user_id: row.get("user_id"), 
                is_default: row.get("is_default"),
            });
        }
        None
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
