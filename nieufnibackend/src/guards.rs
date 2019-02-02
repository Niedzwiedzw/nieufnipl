use rocket::{
    Outcome,
    request::{self, FromRequest, Request},
    http::Status,
};
use crate::config::is_crawler;
use crate::config::STATIC_FILE_POSTFIXES;

use crate::models::Author;

const FORBIDDEN: (Status, ()) = (Status::Forbidden, ());

#[derive(Debug)]
pub struct WebCrawlerAgent;

impl<'a, 'r> FromRequest<'a, 'r> for WebCrawlerAgent {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let keys: Vec<_> = request.headers()
            .get("User-Agent")
            .filter(is_crawler)
            .collect();

        if keys.len() > 0 { Outcome::Success(Self) } else { Outcome::Forward(()) }
    }
}

fn is_static_file(path: &str) -> bool {
    STATIC_FILE_POSTFIXES.iter().any(|postfix| path.ends_with(postfix))
}

pub struct NotStaticFile;

impl<'a, 'r> FromRequest<'a, 'r> for NotStaticFile {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let source_link = request.uri().path();
        if is_static_file(source_link) {
            Outcome::Forward(())
        } else { Outcome::Success(Self) }
    }
}

pub struct AuthenticatedUser(Author);


impl <'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let username = match request.headers().get("not-trustworthy-username").last() {
            Some(n) => dbg!(n),
            None => return Outcome::Failure(FORBIDDEN),
        };

        let password = match request.headers().get("not-trustworthy-password").last() {
            Some(p) => dbg!(p),
            None => return Outcome::Failure(FORBIDDEN),
        };

        match Author::login(String::from(username), String::from(password)) {
            Ok(author) => Outcome::Success(Self(author)),
            Err(_e) => Outcome::Failure(FORBIDDEN),
        }
    }
}

impl Into<Author> for AuthenticatedUser {
    fn into(self) -> Author {
        self.0
    }
}
