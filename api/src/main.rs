#![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket::{get, routes};
use rocket_contrib::serve::StaticFiles;

fn static_files(rocket: Rocket) -> Result<Rocket, Rocket> {
    const DEFAULT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

    let dir = rocket
        .config()
        .get_str("static_dir")
        .unwrap_or(DEFAULT_DIR)
        .to_string();

    let static_files = StaticFiles::from(dir);

    let rocket = rocket.mount("/", static_files);

    Ok(rocket)
}

#[get("/")]
fn index() -> &'static str {
    "Hello rocket!"
}

fn ignite() -> rocket::Rocket {
    rocket::ignite()
        .attach(AdHoc::on_attach("Static files", static_files))
        .mount("/", routes![index])
}

fn main() {
    env_logger::init();
    ignite().launch();
}