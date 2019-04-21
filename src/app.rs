use crate::{Config, Site};

#[derive(Clone)]
pub struct App {
    config: Config,
    site: Site,
}

impl App {
    pub fn new(config: Config, site: Site) -> App {
        App {
            config: config,
            site: site,
        }
    }
}