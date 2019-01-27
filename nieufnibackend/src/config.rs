pub const ARTICLES_PATH: &str = "./articles/";
pub const METADATA_SPLIT: &str = "-METADANE-";
pub const CRAWLER_PREFIXES: [&'static str; 1] = ["facebookexternalhit"];

pub fn is_crawler(user_agent: &&str) -> bool {
    CRAWLER_PREFIXES.iter().any(|entry| user_agent.starts_with(entry))
}
