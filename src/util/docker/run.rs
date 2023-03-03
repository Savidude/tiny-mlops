use shiplift::{Docker, ContainerOptions, builder::ContainerOptionsBuilder};
use tokio;

use crate::model::image::image;

use log::{info, debug, error};

#[tokio::main]
pub async fn run_image(image: &image::Image) {
    let docker = Docker::new();
    let options = ContainerOptions::builder(&image.repository)
        // .cmd(vec!["echo", "hello world"])
        .build();
    let container = docker
        .containers()
        .create(&options)
        .await;
    match container {
        Ok(container_info) => {
            let start_result = docker
                .containers()
                .get(&container_info.id)
                .start()
                .await;
            match start_result {
                Ok(_info) => info!("{} is running!", image.repository),
                Err(e) => error!("Error while starting image {}: {}", image.repository, e),
            }
        },
        Err(e) => eprintln!("Error: {}", e),

    }
}

#[tokio::main]
pub async fn run_image_with_args(image: &image::Image, args: &image::ImageArgs) {
    let docker = Docker::new();
    let options = build_container_options(image, args).build();
    let container = docker
        .containers()
        .create(&options)
        .await;
    match container {
        Ok(container_info) => {
            let start_result = docker
                .containers()
                .get(&container_info.id)
                .start()
                .await;
            match start_result {
                Ok(_info) => info!("{} is running!", image.repository),
                Err(e) => error!("Error while starting image {}: {}", image.repository, e),
            }
        },
        Err(e) => eprintln!("Error: {}", e),

    }
}

fn build_container_options(image: &image::Image, args: &image::ImageArgs) -> ContainerOptionsBuilder {
    debug!("Building container options...");
    let mut options = ContainerOptions::builder(&image.repository);
    match &args.expose {
        Some(x) => {
            options.expose(x.srcport, &x.protocol, x.hostport);
        },
        None    => {
            debug!("No expose argument");
        },
    }
    match &args.volumes {
        Some(x) => {
            options.volumes(vec![x]);
        },
        None    => {
            debug!("No volumes argument");
        },
    }
    return options;
}
