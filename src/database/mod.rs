use crate::Error;
use crate::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub struct Database {
    pub db: Surreal<Client>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let db = Surreal::new::<Ws>("127.0.0.1:8001").await?;
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
        db.use_ns("todo").use_db("todo").await?;

        Ok(Database { db })
    }

    pub async fn insert<T: Serialize>(&self, table: &str, record: T) -> Result<Record> {
        let created: Vec<Record> = self.db.create(table).content(record).await?;
        created
            .into_iter()
            .next()
            .ok_or(Error::OtherError(format!("Failed to insert data.")))
    }

    pub async fn select<T: DeserializeOwned>(&self, table: &str) -> Result<Vec<T>> {
        let result: Vec<T> = self.db.select(table).await?;
        Ok(result)
    }

    pub async fn delete(&self, table: &str, id: &str) -> Result<Option<Record>> {
        let result: Option<Record> = self.db.delete((table, id)).await?;
        Ok(result)
    }
}
