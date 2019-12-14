#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use std::error::Error;

#[get("/")]
fn root_get() -> Template {
    let empty_hashmap: HashMap<String, String> = HashMap::new();
    Template::render("index", empty_hashmap)
}

fn main() -> Result<(), Box<dyn Error>> {

    rocket::ignite()
        .mount("/", routes![root_get])
        .mount("/", StaticFiles::from("build"))
        .attach(Template::fairing())
        .launch();

    Ok(())
}

