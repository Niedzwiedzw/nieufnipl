#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{
    get, ignite, routes,
    http::uri::Segments,
    response::content::Html,
};
use rocket_contrib::{
    json::Json,
    serve::StaticFiles,
};
use crate::guards::WebCrawlerAgent;
use crate::filesystem::index_file;
use crate::guards::NotStaticFile;
use crate::crawler_handling::crawler_response;

mod models;
mod filesystem;
mod config;
mod guards;
mod crawler_handling;

#[get("/")]
fn api_index() -> &'static str {
    "Nieufne API 0.1!"
}

#[get("/<path..>", rank=0)]
fn crawler_metadata(path: Segments, _agent: WebCrawlerAgent) -> Html<String> {
    let article_id = path.last().expect("misformatted link");
    let article = match models::Article::get(article_id) {
        Some(a) => a,
        None => models::Article::bad_article(),
    };

    let response = crawler_response(
        String::from("article"),
        String::from("https://i.ytimg.com/vi/W9t6GZ0vNPA/hqdefault.jpg"),
        article.title.clone(),
        format!("www.nieufni.pl/{}", article.id),
        article.rendered_text.clone()
    );

    Html(response)
}

#[get("/<path..>", rank=9)]
fn home(path: Segments, _is_static: NotStaticFile) -> Html<String> {
    Html(index_file())
}

#[get("/artykuly/<id>")]
fn article(id: String) -> Option<Json<models::Article>> {
    match models::Article::get(&id) {
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
