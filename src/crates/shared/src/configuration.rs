use std::fs;
use lazy_static::lazy_static;
use homedir::my_home;
use serde::{Serialize, Deserialize};
use toml;

lazy_static! {
    pub static ref CONFIG: Config = load_or_create_config();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_lang")] 
    pub lang: String,
    
    pub logging: Logging,
    
    #[serde(default = "default_config_path")]
    pub path_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logging {
    pub level: LoggingLevel,
    #[serde(alias = "path")]
    pub folder: String,
    pub stdout: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LoggingLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

fn default_lang() -> String {
    "EN".to_string()
}

fn default_config_path() -> String {
    ".config/usqlrepl/config.toml".to_string()
}

impl std::fmt::Display for Logging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Logging(level={}, folder=\"{}\", stdout={})",
            self.level, self.folder, self.stdout
        )
    }
}

impl Default for Logging {
    fn default() -> Self {
        let home = my_home().unwrap().unwrap();
        let folder = format!("{}/.local/share/usqlrepl/logs", home.to_str().unwrap());
        Self {
            level: LoggingLevel::Info,
            folder,
            stdout: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lang: default_lang(),
            logging: Logging::default(),
            path_config: default_config_path(),
        }
    }
}

pub fn load_or_create_config() -> Config {
    let home = my_home().unwrap().unwrap();
    let config_path = home.join(".config/usqlrepl/config.toml");

    if config_path.exists() {
        let content = fs::read_to_string(&config_path).expect("Cannot read config");
        match toml::from_str(&content) {
            Ok(cfg) => return cfg,
            Err(e) => {
                eprintln!("Warning: Failed to parse config ({}), creating new one.", e);
            }
        }
    }

    let cfg = Config::default();
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let toml_string = toml::to_string_pretty(&cfg).unwrap();
    fs::write(&config_path, toml_string).unwrap();
    cfg
}

impl std::fmt::Display for LoggingLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LoggingLevel::Error => "error",
            LoggingLevel::Warn => "warn",
            LoggingLevel::Info => "info",
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

impl Logging {
    pub fn get_folder(&self) -> Option<std::path::PathBuf> {
        Some(std::path::PathBuf::from(&self.folder))
    }
    pub fn ensure_log_dir(path: &std::path::PathBuf) {
        std::fs::create_dir_all(path).expect("Failed to create log directory");
    }
}