use log::{info, error};

use crate::model::image::image;

use crate::util::docker;
use crate::constants;
use crate::util::system;
use crate::util::files;

fn get_mosquitto_conf(mqtt_config_file: &str) {
    let version: &str = env!("CARGO_PKG_VERSION");
    let conf_url: String = format!("{}/v{}/{}", constants::GITHUB_RAW_CONTENT_PROJECT_URL, version, constants::MQTT_BROKER_CONF_FILE);
    if let Err(e) = files::download_from_url(&conf_url, &mqtt_config_file) {
        error!("Error while downloading file {}", e);
    }
}

pub fn start_mqtt_broker() {
    info!("Pulling MQTT boker...");
    let broker_image = image::Image { host: constants::DOCKERHUB_URL.to_string(), username: "".to_string(), 
                                                password: "".to_string(), repository: constants::MQTT_BROKER_CONTAINER.to_string() };
    docker::pull::pull_image(&broker_image);

    let edge_home = system::get_edge_home_dir();
    let config_file_download_path = format!("{}/{}", edge_home, constants::MQTT_BROKER_CONF_FILE);
    info!("Downloading MQTT broker config...");
    get_mosquitto_conf(&config_file_download_path);

    info!("Starting MQTT boker...");
    let expose_arg = image::ArgExpose { srcport:1883, protocol:"tcp".to_string(), hostport:1883};
    let volumes = format!("{}:{}", config_file_download_path, constants::MQTT_BROKER_CONFIG_PATH);
    let image_args = image::ImageArgs { expose: Some(expose_arg), volumes: Some(volumes.to_string()), add_host:None };
    docker::run::run_image_with_args(&broker_image, &image_args);
}

