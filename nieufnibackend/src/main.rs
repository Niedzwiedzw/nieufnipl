#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

use std::error::Error;

use rocket::{
    get, post, ignite, routes,
    http::{uri::Segments, Status},
    response::content::Html,
};
use rocket_contrib::{
    json::Json,
    serve::StaticFiles,
};

#[macro_use]
extern crate diesel;
extern crate dotenv;
mod schema;

#[allow(unused_imports)]
#[macro_use] extern crate slugify;

use crate::guards::WebCrawlerAgent;
use crate::filesystem::index_file;
use crate::guards::NotStaticFile;
use crate::crawler_handling::crawler_response;
use crate::models::NewAuthor;
use crate::models::Author;
use crate::guards::AuthenticatedUser;


mod models;
mod filesystem;
mod config;
mod guards;
mod crawler_handling;
mod database;
mod time_handling;
mod helpers;


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
#[allow(unused_variables)]
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

#[post("/artykuly", data = "<article>", format="application/json")]
fn new_article(article: Json<models::NewArticle>, user: AuthenticatedUser) -> Status {
    let mut article = article.into_inner();
    let user: Author = user.into();

    article.authors_id = user.id;
    if let Some(_) = article.save() {
        Status::Created
    } else { Status::Conflict }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1) {
        Some(arg) => {
            match arg.as_ref() {
                first_argument => {
                    match first_argument {
                        "--create-author" => {
                            let (username, password) = (
                                args.get(2).expect("username needed"),
                                args.get(3).expect("password needed"),
                            );
                            NewAuthor::create(username.clone(), password.clone())?;
                            println!("created {}!", &username);
                            return Ok(())
                        },
                        "--list-authors" => {
                            println!("users: {:#?}", Author::all());
                            return Ok(())
                        },
                        _ => panic!("invalid argument!"),
                    }
                }
            }
        },
        None => {}
    }


    ignite()
        .mount("/", StaticFiles::from("../nieufnifront/dev"))
        .mount("/", routes![home])
        .mount("/", routes![crawler_metadata])
        .mount("/api/", routes![api_index, article, all_articles, new_article])
        .launch();

    Ok(())
}
