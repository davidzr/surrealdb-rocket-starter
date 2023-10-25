use crate::models::task::{Task, TaskResult};
use crate::{database, Result};
use rocket::serde::json::{json, Json, Value};
use rocket::State;

#[get("/tasks")]
pub async fn tasks(db: &State<database::Database>) -> Result<Value> {
    let tasks: Vec<TaskResult> = db.select("tasks").await?;
    Ok(json!({"result":tasks}))
}

#[post("/tasks", format = "json", data = "<task>")]
pub async fn tasks_create(db: &State<database::Database>, task: Json<Task>) -> Result<Value> {
    let created = db.insert("tasks", task.into_inner()).await?;
    Ok(json!({"created": created}))
}

#[post("/tasks/delete/<id>")]
pub async fn tasks_delete(db: &State<database::Database>, id: &str) -> Value {
    let created = db.delete("tasks", id).await;
    match created {
        Ok(_) => {
            json!({"deleted": true})
        }
        Err(_) => json!({"deleted": false}),
    }
}