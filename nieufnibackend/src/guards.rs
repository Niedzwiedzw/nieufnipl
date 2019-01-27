use rocket::{
    Outcome,
    request::{self, FromRequest, Request}
};
use crate::config::is_crawler;

#[derive(Debug)]
pub struct WebCrawlerAgent;


impl<'a, 'r> FromRequest<'a, 'r> for WebCrawlerAgent {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let source_link = String::from(request.uri().path());
        let keys: Vec<_> = request.headers()
            .get("User-Agent")
            .filter(is_crawler)
            .collect();

        if keys.len() > 0 { Outcome::Success(Self) } else { Outcome::Forward(()) }
    }
}