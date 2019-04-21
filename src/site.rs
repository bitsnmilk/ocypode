use crate::Config;

#[derive(Clone)]
pub struct Site {
    config: Config,
}

impl Site {
    pub fn new(config: Config) -> Site {
        Site { config }
    }

    fn build_response(&self) -> Result<String, ()> {
        Ok(String::from("Hello world!"))
    }
}