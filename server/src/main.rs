#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;


use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}

#[get("/<path..>")]
fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}



fn ignite() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, static_file])
}


fn main() {
    ignite().launch();
}

