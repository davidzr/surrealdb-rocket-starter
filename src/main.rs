#[macro_use]
extern crate rocket;
mod database;
mod error;
mod routes;
mod models;
pub use error::{Error, Result};
use rocket_dyn_templates::Template;
#[launch]
async fn rocket() -> _ {
    let db = database::Database::new().await.unwrap();
    rocket::build()
        .mount(
            "/",
            routes![
                routes::main::index,
            ],
        )
        .mount(
            "/api",
            routes![
                routes::tasks::tasks,
                routes::tasks::tasks_create,
                routes::tasks::tasks_delete
            ],
        )
        .manage(db)
        .attach(Template::fairing())
}
