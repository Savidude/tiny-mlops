use std::io;
use std::io::Write;
use std::process;

use log::{info, debug, error};
use serde::Deserialize;
use serde::Serialize;

use crate::util::system;
use crate::util::files;
use crate::constants;

use crate::results::FileCreateResult;

#[derive(Debug, Deserialize, Serialize)]
struct Model {
    host: String,
    username: String,
    password: String,
    repository: String,
}

impl Default for Model {
    fn default () -> Model {
        Model{host: "".to_owned(), username: "".to_owned(), password: "".to_owned(), repository: "".to_owned()}
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    fleet_id: String,
    model: Model,
}

impl Default for Config {
    fn default () -> Config {
        Config{fleet_id: "".to_owned(), model: Model::default()}
    }
}

pub fn initialize(id: Option<String>, model_host: Option<String>, model_username: Option<String>, model_passwd: Option<String>, model_repo: Option<String>) {
    let mut config: Config = read_config();
    if config.fleet_id == "" {
        config = prompt_config_values(config, id, model_host, model_username, model_passwd, model_repo);
        create_config(config);
    } else {
        info!("Device is already initialized")
    }
}

fn read_config() -> Config {
    debug!("Reading config file...");

    let home_dir = system::get_home_dir();
    let edge_home = &format!("{}/{}", home_dir, constants::EDGE_HOME_DIR);
    let config_file_path = &format!("{}/{}", edge_home, constants::CONFIG_FILE);
    
    if files::file_exists(config_file_path) {
        let config: Config = files::read_json(config_file_path).unwrap();
        config
    } else {
        Config::default()
    }
}

fn prompt_config_values(mut config: Config, id: Option<String>, model_host: Option<String>, model_username: Option<String>, 
                        model_passwd: Option<String>, model_repo: Option<String>) -> Config {
    match id {
        Some(x) => {config.fleet_id = x},
        None    => {
            print!("Fleet ID: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut config.fleet_id).expect("Didn't Receive Input");
            config.fleet_id = config.fleet_id.trim_end().to_owned()
        },
    }
    match model_host {
        Some(x) => {config.model.host = x},
        None    => {
            print!("Model registry host (https://registry.hub.docker.com/v2): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut config.model.host).expect("Didn't Receive Input");
            config.model.host = config.model.host.trim_end().to_owned()
        },
    }
    match model_username {
        Some(x) => {config.model.username = x},
        None    => {
            print!("Model registry username: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut config.model.username).expect("Didn't Receive Input");
            config.model.username = config.model.username.trim_end().to_owned()
        },
    }
    match model_passwd {
        Some(x) => {config.model.password = x},
        None    => {
            print!("Model registry password: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut config.model.password).expect("Didn't Receive Input");
            config.model.password = config.model.password.trim_end().to_owned()
        },
    }
    match model_repo {
        Some(x) => {config.model.repository = x},
        None    => {
            print!("Model repository: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut config.model.repository).expect("Didn't Receive Input");
            config.model.repository = config.model.repository.trim_end().to_owned()
        },
    }
    return config;
}

fn create_config(config: Config) {
    debug!("Creating config file...");

    let home_dir = system::get_home_dir();
    let edge_home = &format!("{}/{}", home_dir, constants::EDGE_HOME_DIR);
    create_home_dir(edge_home);
    
    let config_file_path = &format!("{}/{}", edge_home, constants::CONFIG_FILE);
    create_config_file(config_file_path, config);
}

fn create_config_file(config_file_path: &str, config: Config) {
    debug!("Creating config file...");
    files::write_to_json(&config, config_file_path).unwrap();
    debug!("Config file created");
}

fn create_home_dir(dir_path: &str){
    match files::create_dir_if_exists(dir_path) {
        FileCreateResult::OK() => {
            debug!("Created home directory at {}", dir_path);
        },
        FileCreateResult::FileExists() => {
            debug!("Home directory already exists. Skipping...");
        },
        FileCreateResult::Error(_) => {
            error!("Error while creating home directory");
            process::exit(constants::EXIT_CODE_SYSTEM_ERR);
        },
    }
}
