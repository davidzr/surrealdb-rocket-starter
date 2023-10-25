use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! { field: "value" })
}