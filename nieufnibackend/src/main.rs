#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, ignite, routes };
use rocket_contrib::{
    json::Json,
    serve::StaticFiles,
};
use serde::{Deserialize, Serialize};

use std::io::{self, Write};

mod models;
mod filesystem;
mod config;

#[get("/")]
fn index() -> &'static str {
    "Nieufne API 0.1!"
}

#[get("/artykuly/<id>")]
fn article(id: String) -> Option<Json<models::Article>> {
    match models::Article::all_articles().into_iter().find(|a| a.id == id) {
        Some(article) => Some(Json(article)),
        None =>  None,
    }
}

#[get("/artykuly")]
fn all_articles() -> Json<Vec<models::Article>> {
    Json(models::Article::all_articles())
}

fn main() {
    ignite()
        .mount("/", StaticFiles::from("../nieufnifront/dev"))
        .mount("/api/", routes![index, article, all_articles])
        .launch();
}
