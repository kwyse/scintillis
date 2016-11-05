//! Represents a YAML-based configuration file with optional overrides
//! passed in via the command line.

use clap::App;
use serde_yaml;
use std::error::Error;
use std::io;
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
    pub frame_rate: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            window_width: 640,
            window_height: 480,
            frame_rate: 60.0,
        }
    }
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    use std::fs::File;

    let config_file = try!(File::open(path));
    let config = try!(serde_yaml::from_reader(config_file));

    Ok(config)
}

pub fn apply_session_overrides(mut config: Config) -> Config {
    let overrides = get_defined_cli().get_matches();
    let overridden_value = |arg| overrides.value_of(arg).and_then(|val| val.parse::<u32>().ok());

    if let Some(new_width) = overridden_value("width") { config.window_width = new_width }
    if let Some(new_height) = overridden_value("height") { config.window_height = new_height }

    config
}

fn get_defined_cli<'a, 'b>() -> App<'a, 'b> {
    use clap::Arg;

    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("width")
             .short("W")
             .long("width")
             .value_name("VALUE")
             .help("Sets the width of the window")
             .takes_value(true))
        .arg(Arg::with_name("height")
             .short("H")
             .long("height")
             .value_name("VALUE")
             .help("Sets the height of the window")
             .takes_value(true))
}

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Parse(serde_yaml::Error),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        ConfigError::Parse(err)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::Io(ref err) => err.fmt(f),
            ConfigError::Parse(ref err) => err.fmt(f)
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::Io(ref err) => err.description(),
            ConfigError::Parse(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConfigError::Io(ref err) => Some(err),
            ConfigError::Parse(ref err) => Some(err)
        }
    }
}
