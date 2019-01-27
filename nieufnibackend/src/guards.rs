use rocket::{
    Outcome,
    request::{self, FromRequest, Request}
};
use crate::config::is_crawler;
use crate::config::STATIC_FILE_POSTFIXES;

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