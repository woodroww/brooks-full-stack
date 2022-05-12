use crate::database::{TaskId, TodoDB, UserId};
use crate::routes::tasks::{CreateTaskRequest, Task, TaskInfo};
use chrono::NaiveDateTime;

impl TodoDB {
    pub async fn insert_task(&self, task: &CreateTaskRequest, user_id: UserId) -> Option<TaskInfo> {
        let con = self.pool.get().await.unwrap();
        let sql = "INSERT INTO tasks (title, description, user_id) VALUES ($1, $2, $3) RETURNING id, priority, title, completed_at, description";
        let err = con
            .query(sql, &[&task.title, &task.description.clone(), &user_id])
            .await;
        if err.is_err() {
            return None;
        }
        let query_result = err.ok().unwrap();
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

    pub async fn get_all_tasks(&self, user_id: UserId) -> Option<Vec<TaskInfo>> {
        let con = self.pool.get().await.unwrap();
        let sql =
            "SELECT completed_at, description, id, priority, title FROM tasks WHERE user_id = $1";
        let err = con.query(sql, &[&user_id]).await;
        if err.is_err() {
            return None;
        }

        let query_result = err.ok().unwrap();
        let mut results = vec![];
        for row in query_result {
            results.push(TaskInfo {
                id: row.get("id"),
                priority: row.get("priority"),
                title: row.get("title"),
                completed_at: row.get("completed_at"),
                description: row.get("description"),
            });
        }
        Some(results)
    }

    pub async fn get_task(&self, user_id: UserId, task_id: TaskId) -> Option<Task> {
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

    pub async fn mark_completed(&self, user_id: UserId, task_id: TaskId) -> bool {
        let completed = Some(chrono::Local::now().naive_local());
        self.update_completed_status(user_id, task_id, completed)
            .await
    }

    pub async fn mark_uncompleted(&self, user_id: UserId, task_id: TaskId) -> bool {
        self.update_completed_status(user_id, task_id, None).await
    }

    async fn update_completed_status(
        &self,
        user_id: UserId,
        task_id: TaskId,
        completed_at: Option<NaiveDateTime>,
    ) -> bool {
        let con = self.pool.get().await.unwrap();
        let sql = "UPDATE tasks SET completed_at = $1 WHERE user_id = $2 AND id = $3 AND deleted_at = NULL";
        let err = con.query(sql, &[&completed_at, &user_id, &task_id]).await;
        if err.is_err() {
            let e = err.err().unwrap();
            if let Some(db_err) = e.as_db_error() {
                println!("db_get_task error {}", db_err.message().to_string());
            } else {
                println!("where is the error");
            }
            return false;
        }
        true
    }

    pub async fn update_task(&self, task: &TaskInfo, user_id: UserId) -> Option<TaskInfo> {
        let con = self.pool.get().await.unwrap();
        let sql = r#"
            UPDATE tasks 
            SET (completed_at, priority, title, description) = ($1, $2, $3, $4) 
            WHERE id = $5 AND deleted_at is NULL AND user_id = $6
            RETURNING id, priority, title, completed_at, description
            "#;
        let err = con
            .query(
                sql,
                &[
                    &task.completed_at,
                    &task.priority,
                    &task.title,
                    &task.description,
                    &task.id,
                    &user_id,
                ],
            )
            .await;
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
        if query_result.len() == 0 {
            println!("so returning is not returning anything!!!!");
        }

        if let Some(row) = query_result.first() {
            Some(TaskInfo {
                id: row.get("id"),
                priority: row.get("priority"),
                title: row.get("title"),
                completed_at: row.get("completed_at"),
                description: row.get("description"),
            })
        } else {
            None
        }
    }

    pub async fn soft_delete_task(&self, user_id: UserId, task_id: TaskId) -> bool {
        let con = self.pool.get().await.unwrap();
        let sql = r#"
            UPDATE tasks 
            SET (deleted_at) = ($1) 
            WHERE id = $2 AND user_id = $3 AND deleted_at is NULL
            "#;
        let time = chrono::Utc::now().naive_local();
        let err = con.query(sql, &[&time, &task_id, &user_id]).await;
        if err.is_err() {
            let e = err.err().unwrap();
            if let Some(db_err) = e.as_db_error() {
                println!("db_get_task error {}", db_err.message().to_string());
            } else {
                println!("where is the error");
            }
            return false;
        }
        true
    }

    pub async fn get_default_tasks(&self) -> Option<Vec<TaskInfo>> {
        // return db.select().from("tasks").where({ is_default: true });
        let con = self.pool.get().await.unwrap();
        let sql = r#" SELECT * FROM tasks WHERE is_default is true"#;
        let err = con.query(sql, &[]).await;
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
        let mut tasks = vec![];
        for row in query_result {
            tasks.push(TaskInfo {
                id: row.get("id"),
                priority: row.get("priority"),
                title: row.get("title"),
                completed_at: row.get("completed_at"),
                description: row.get("description"),
            });
        }
        Some(tasks)
    }
}
