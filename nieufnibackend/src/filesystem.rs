use std::fs::read_dir;
use crate::config;

//pub fn all_files() -> Vec<String> {
//    let mut articles: Vec<String> = vec![];
//    for entry in read_dir(config::ARTICLES_PATH).expect("Failed to read artcles dir.") {
//        articles.push(entry.unwrap().path().display().to_string());
//    }
//
//    articles
//}

pub fn article_files() -> Vec<String> {
    read_dir(config::ARTICLES_PATH).expect("Failed to read articles directory.")
        .into_iter()
        .map(|entry| entry.unwrap().path().display().to_string())
        .filter(|path| path.ends_with(".md"))
        .collect()
}


