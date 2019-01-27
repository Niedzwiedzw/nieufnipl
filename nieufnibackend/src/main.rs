#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{
    get, ignite, routes,
    http::uri::Segments,
    Catcher,
    response::content::Html,
};
use rocket_contrib::{
    json::Json,
    serve::StaticFiles,
};
use crate::guards::WebCrawlerAgent;
use crate::filesystem::index_file;

mod models;
mod filesystem;
mod config;
mod guards;

#[get("/")]
fn api_index() -> &'static str {
    "Nieufne API 0.1!"
}

#[get("/", rank=0)]
fn crawler_metadata(agent: WebCrawlerAgent) -> &'static str {
    "Hello bot..."
}

#[get("/<path..>", rank=99)]
fn home(path: Segments) -> Html<String> {
    Html(index_file())
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
        .mount("/", routes![home])
        .mount("/", routes![crawler_metadata])
        .mount("/api/", routes![api_index, article, all_articles])
        .launch();
}
