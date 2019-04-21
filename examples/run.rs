extern crate ocypode;

use ocypode::{App, Config, Ocypode, Site};

fn main() {
    let config = Config::new();
    let site = Site::new(config.clone());
    let app = App::new(config.clone(), site);
    let ocypode = Ocypode::new(app);

    ocypode.start(3000);
}
