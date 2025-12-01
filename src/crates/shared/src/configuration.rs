use std::path::PathBuf;

use lazy_static::lazy_static;
use homedir::my_home;

lazy_static! {
  pub static ref CONFIG: Config = load_config();
}

fn load_config() -> Config {
  let config_path = format!(
      "{}/.config/asya/config.lua",
      my_home().expect("Cant find directory!")
               .expect("Directory was not find!")
            .to_str().unwrap().to_string()
  );

  Config { logging: Logging::default(), path: config_path }
}

pub fn load_any_file(pathes: Vec<String>) -> Option<(String, String)> {
  pathes.into_iter().find_map(|path| {
      std::fs::read_to_string(&path)
          .map(|content| (path, content))
          .ok()
  })
}

#[derive(Debug)]
#[derive(Default, Clone)]
pub struct Config {
  pub logging: Logging,
  pub path: String,
}

#[derive(Debug)]
#[derive(Default, Clone)]
pub struct Logging {
    pub level: LoggingLevel,
    pub folder: String,
    pub stdout: bool,
}

impl Logging {
    pub fn get_folder(&self) -> Option<PathBuf> {
        Some(PathBuf::from(&self.folder))
    }
}

#[derive(Clone, Debug)]
pub enum LoggingLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

///
/// DEFAULT OPTIONS
/// 

impl Default for LoggingLevel {
    fn default() -> Self {
        LoggingLevel::Info
    }
}

impl From<String> for LoggingLevel {
    fn from(value: String) -> Self {
        match value.as_str().to_lowercase().as_str() {
            "error" => LoggingLevel::Error,
            "warn" => LoggingLevel::Warn,
            "info" => LoggingLevel::Info,
            "debug" => LoggingLevel::Debug,
            "trace" => LoggingLevel::Trace,
            _ => panic!("Cannot recognize logging level: {}", value),
        }
    }
}