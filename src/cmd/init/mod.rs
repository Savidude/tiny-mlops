use std::io;
use std::io::Write;
use std::process;

use log::{info, debug, error};

use crate::model::config::Config;
use crate::model::image::image;

use crate::util::docker;
use crate::util::system;
use crate::util::mqtt;
use crate::util::files;
use crate::constants;

use crate::results::FileCreateResult;

pub fn initialize(id: Option<String>, model_host: Option<String>, model_username: Option<String>, model_passwd: Option<String>, model_repo: Option<String>) {
    let mut config: Config::Config = read_config();
    if config.fleet_id == "" {
        config = prompt_config_values(config, id, model_host, model_username, model_passwd, model_repo);
        create_config(&config);
        mqtt::broker::start_mqtt_broker();
        start_model(&config.model);
    } else {
        info!("Device is already initialized");
    }
}

fn read_config() -> Config::Config {
    debug!("Reading config file...");
    let edge_home = system::get_edge_home_dir();
    let config_file_path = &format!("{}/{}", edge_home, constants::CONFIG_FILE);
    
    if files::file_exists(config_file_path) {
        let config: Config::Config = files::read_json(config_file_path).unwrap();
        config
    } else {
        Config::default()
    }
}

fn prompt_config_values(mut config: Config::Config, id: Option<String>, model_host: Option<String>, model_username: Option<String>, 
                        model_passwd: Option<String>, model_repo: Option<String>) -> Config::Config {
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
            print!("Model registry host (registry.hub.docker.com): ");
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

fn create_config(config: &Config::Config) {
    debug!("Creating config file...");
    let edge_home = system::get_edge_home_dir();
    create_home_dir(&edge_home);
    
    let config_file_path = &format!("{}/{}", edge_home, constants::CONFIG_FILE);
    create_config_file(config_file_path, config);
}

fn create_config_file(config_file_path: &str, config: &Config::Config) {
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

fn start_model(image: &image::Image) {
    info!("Pulling model image");
    docker::pull::pull_image(image);
    info!("Starting model image...");
    let image_args = image::ImageArgs { expose: None, volumes: None, add_host: Some(constants::DOCKER_LOCALHOST.to_string()) };
    docker::run::run_image_with_args(&image, &image_args);
    info!("The model is running...");
}
