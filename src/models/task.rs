use crate::{Error, Result};
use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    description: String,
    completed: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskResult {
    id: Thing,
    description: String,
    completed: bool,
}

impl TryFrom<TaskResult> for Task {
    type Error = Error;

    fn try_from(value: TaskResult) -> Result<Self> {
        let task = Task {
            description: value.description,
            completed: value.completed,
        };
        Ok(task)
    }
}
