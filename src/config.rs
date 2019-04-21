use std::env::{current_dir, var};
use std::ffi::OsStr;

#[derive(Clone)]
pub struct SummaryConfig(u32, char);

impl SummaryConfig {
    pub fn new(limit: u32, delimeter: char) -> SummaryConfig {
        SummaryConfig(limit, delimeter)
    }
}

#[derive(Clone)]
pub struct Config {
    author: String,
    title: String,
    root: String,
    date_formatter: fn(String) -> String,
    summary: SummaryConfig,
    ext: String,
    cache: u32,
}

impl Config {
    pub fn new() -> Config {
        Config {
            author: var("USER").unwrap_or(String::from("Krusty")),
            title: Config::get_default_title(),
            root: String::from("index"),
            date_formatter: |a| a,
            summary: SummaryConfig::new(150, '~'),
            ext: String::from("md"),
            cache: 1500,
        }
    }

    fn get_default_title() -> String {
        match current_dir() {
            Ok(dir) => dir
                .file_name()
                .and_then(OsStr::to_str)
                .map(String::from)
                .unwrap_or(String::from("test")),
            _ => String::from("Ocypode"),
        }
    }

    pub fn author(&self) -> &String {
        &self.author
    }
}