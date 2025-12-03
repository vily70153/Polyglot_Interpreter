use std::path::PathBuf;

use lazy_static::lazy_static;
use homedir::my_home;

use std::fmt;

lazy_static! {
  pub static ref CONFIG: Config = load_config();
}

fn load_config() -> Config {
    let home = my_home()
        .expect("home missing")
        .expect("home missing");

    let config_path = home.join(".config/usqlrepl/config.toml");

    Config {
        logging: Logging::default(),
        path: config_path.to_string_lossy().to_string(),
    }
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
#[derive(Clone)]
pub struct Logging {
    pub level: LoggingLevel,
    pub folder: String,
    pub stdout: bool,
}

impl Logging {
    pub fn get_folder(&self) -> Option<PathBuf> {
        Some(PathBuf::from(&self.folder))
    }

    pub fn ensure_log_dir(path: &PathBuf) {
        std::fs::create_dir_all(path)
            .expect("Failed to create log directory");
    }
    
}

impl Default for Logging {
    fn default() -> Self {
        let home = my_home()
            .expect("home dir missing")
            .expect("home path missing");

        let default_folder = format!(
            "{}/.local/share/usqlrepl/logs",
            home.to_str().unwrap()
        );

        Self {
            level: LoggingLevel::Info,
            folder: default_folder,
            stdout: true,
        }
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

impl Into<String> for LoggingLevel {
    fn into(self) -> String {
        match self {
            LoggingLevel::Error => "error".into(),
            LoggingLevel::Warn  => "warn".into(),
            LoggingLevel::Info  => "info".into(),
            LoggingLevel::Debug => "debug".into(),
            LoggingLevel::Trace => "trace".into(),
        }
    }
}

impl fmt::Display for LoggingLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LoggingLevel::Error => "error",
            LoggingLevel::Warn  => "warn",
            LoggingLevel::Info  => "info",
            LoggingLevel::Debug => "debug",
            LoggingLevel::Trace => "trace",
        };
        write!(f, "{}", s)
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