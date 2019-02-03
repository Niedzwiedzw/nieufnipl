use std::fmt;

use serde::{Deserialize, Serialize};
use comrak::{ markdown_to_html, ComrakOptions };
use diesel::prelude::*;
use regex::Regex;

use crate::schema::articles::dsl as articles_dsl;
use crate::schema::authors::dsl as authors_dsl;

use crate::database::establish_connection;
use crate::schema::{ articles, authors };

pub fn render_markdown(content: &String) -> String {
    markdown_to_html(
        content.as_str(),
        &ComrakOptions::default()
    )
}

use slugify::slugify;
use std::error::Error;
use crate::helpers::to_hash;

#[derive(Debug)]
pub struct ValidationError(String);

impl ValidationError {
    pub fn new(msg: String) -> Self { Self(msg) }
    pub fn boxed(msg: String) -> Box<Self> { Box::new(Self::new(msg)) }
}

impl Error for ValidationError {
    fn description(&self) -> &str { &self.0 }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.0)
    }
}

type ValidationResult<T> = Result<(), Box<T>>;

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name="articles"]
pub struct NewArticle {
    id: String,  // empty
    title: String,
    pub authors_id: i32,  // empty
    date: String,
    markdown_text: String,
    rendered_text: String,  // empty
}

impl NewArticle {
    pub fn validate(&self) -> ValidationResult<dyn Error> {
        let articles = Article::all_articles();
        match articles.iter().find(|a| a.title == self.title) {
            Some(_) => Err(ValidationError::boxed(
                format!("Article with title {} already exists", self.title)
            )),
            None => Ok(())
        }
    }
    pub fn save(&mut self) -> Option<Article> {
        if let Ok(_) = self.validate() {
            let conn = establish_connection();
            let rendered_md = render_markdown(&self.markdown_text);
            let slug = slugify!(&self.title.as_str());
            self.id = slug;
            self.date = crate::time_handling::now();
            self.rendered_text = rendered_md;

            let article = diesel::insert_into(articles_dsl::articles)
                .values(self.clone())
                .get_result(&conn)
                .expect("Error inserting article into database");

            Some(article)
        } else { None }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Queryable)]
#[belongs_to(Author)]
#[table_name="articles"]
pub struct Article {
    pub id: String,
    pub db_id: i32,
    pub title: String,
    pub date: String,
    pub authors_id: i32,
    pub markdown_text: String,
    pub rendered_text: String,
    pub published: bool,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ArticleResponse {
    pub id: String,
    pub title: String,
    pub date: String,
    pub author_name: String,
    pub rendered_text: String,
    pub markdown_text: String,
    pub image: String,
}

impl From<Article> for ArticleResponse {
    fn from(article: Article) -> Self {
        let author = Author::all()
            .into_iter()
            .find(|a| a.id == article.authors_id)
            .expect("Database integrity compromised: could not find associated author.");

        ArticleResponse {
            id: article.id.clone(),
            title: article.title.clone(),
            date: article.date.clone(),
            author_name: author.name.clone(),
            rendered_text: article.rendered_text.clone(),
            markdown_text: article.markdown_text.clone(),
            image: match article.showcase_image() {
                Some(l) => l,
                None => String::from("https://i.ytimg.com/vi/W9t6GZ0vNPA/hqdefault.jpg"),
            }
        }
    }
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
//    pub fn from_file(path: &String) -> Self {
//        let mut file = File::open(path).unwrap();
//        let mut content = String::new();
//
//        file.read_to_string(&mut content).expect("failed to read article file");
//
//        let mut article_data = content.splitn(2,METADATA_SPLIT);
//        let metedata = article_data.next().unwrap();
//        let content = article_data.next().unwrap();
//
//        let (author, date, title) = parse_metadata(metedata);
//
//
//
//        let name = filename(&path);
//
//        Self {
//            id: String::from(article_id(name)),
//            db_id: 0,
//            title,
//            date,
//            author,
//            markdown_text: content.to_string(),
//            rendered_text,
//            published: false,
//        }
//    }

    pub fn all_articles() -> Vec<Self> {
        let connection = establish_connection();

        articles_dsl::articles.load::<Self>(&connection)
            .expect("error loading articles").into_iter().collect()

    }

    pub fn get(id: &str) -> Option<Self> {
        Self::all_articles().into_iter().find(|a| a.id == id)
    }

    pub fn bad_article() -> Self {
        Self::get("zly-artykul").expect("couldn't find a 404 article")
    }

    pub fn showcase_image(&self) -> Option<String> {
        self.images().into_iter().nth(0)
    }

    pub fn images(&self) -> Vec<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(http(s?):)([/|.|\w|\s|-])*\.(?:jpg|gif|png)")
            .expect("regex is correct");
        }

        RE.find_iter(&self.markdown_text.as_str()).map(|m| String::from(m.as_str())).collect()
    }
}


#[derive(Identifiable, Queryable, Insertable, PartialEq, Debug)]
#[table_name="authors"]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
}


impl Author {
    pub fn login(name: String, password: String) -> Result<Self, &'static str> {
        match Self::find(name) {
            Some(author) => {
                if author.password_hash == to_hash(password) {
                    Ok(author)
                } else {
                    Err("Wrong password.")
                }
            },
            None => Err("Wrong username.")
        }

    }

    pub fn all() -> Vec<Author> {
        let connection = establish_connection();
        authors_dsl::authors
            .load::<Self>(&connection)
            .expect("Error while trying to fetch authors.")
            .into_iter()
            .collect()
    }

    pub fn find(name: String) ->Option<Self> {
        Self::all().into_iter().find(|author| author.name == name)
    }
}


#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name="authors"]
pub struct NewAuthor {
    pub name: String,
    pub password_hash: String,
}

impl NewAuthor {
    pub fn new(name: String, password: String) -> Self {
        let password_hash = to_hash(password);
        Self { name, password_hash }
    }

    pub fn create(name: String, password: String) -> Result<(), Box<dyn Error>> {
        let author = Self::new(name, password);
        let connection = establish_connection();
        match diesel::insert_into(authors_dsl::authors)
            .values(&author)
            .get_results::<Author>(&connection) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }

    }
}
