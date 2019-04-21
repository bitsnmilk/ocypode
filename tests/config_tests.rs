use ocypode::Config;
use std::env::var;

#[test]
fn authors_default_value_is_current_user() {
    let config = Config::new();

    let username = var("USER").unwrap();
    assert_eq!(&username, config.author());
}