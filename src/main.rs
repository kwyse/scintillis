#![feature(proc_macro)]

#[macro_use] extern crate clap;
#[macro_use] extern crate glium;
#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;

mod app;
mod config;
mod graphics;

use std::path::Path;

use app::App;

fn main() {
    let config_file = Path::new("config.yml");
    let mut config = config::load_from_file(config_file).ok().unwrap_or_default();
    config = config::apply_session_overrides(config);

    App::from_config(config).run();
}
