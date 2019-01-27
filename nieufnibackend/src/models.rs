use std::io::{Read};
use std::fs::File;

use crate::{
    filesystem::article_files,
    config::METADATA_SPLIT,
};

use serde::{Deserialize, Serialize};
use comrak::{markdown_to_html, ComrakOptions};


#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub markdown_text: String,
    pub rendered_text: String,
}

pub fn filename(path: &String) -> &str {
    let path: Vec<&str> = path.split("/").collect();
    path.last().unwrap()
}

pub fn article_id(filename: &str) -> &str {
    let filename: Vec<&str> = filename.split(".").collect();
    filename.first().unwrap()
}

pub fn parse_metadata(metadata: &str) -> (String, String, String) {
    let mut split = metadata.split("\n");
    let author = split.next().expect("Metadata is misformatted").to_string();
    let date = split.next().expect("Metadata is misformatted").to_string();
    let title = split.next().expect("Metadata is misformatted").to_string();

    (author, date, title)
}

impl Article {
    pub fn from_file(path: &String) -> Self {
        let mut file = File::open(path).unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content).expect("failed to read article file");

        let mut article_data = content.splitn(2,METADATA_SPLIT);
        let metedata = article_data.next().unwrap();
        let content = article_data.next().unwrap();

        let (author, date, title) = parse_metadata(metedata);

        let rendered_text = markdown_to_html(
            content,
            &ComrakOptions::default()
        );

        let name = filename(&path);

        Self {
            id: String::from(article_id(name)),
            title,
            date,
            author,
            markdown_text: content.to_string(),
            rendered_text,

        }
    }

    pub fn all_articles() -> Vec<Self> {
        article_files()
            .iter()
            .map(|file| Self::from_file(file))
            .collect()
    }

    pub fn get(id: &str) -> Option<Self> {
        Self::all_articles().into_iter().find(|a| a.id == id)
    }

    pub fn bad_article() -> Self {
        Self::all_articles()
            .into_iter()
            .find(|article| article.id == "zly-artykul")
            .expect("improperly configured")
    }
}
