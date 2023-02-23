use shiplift::{Docker, ContainerOptions};
use tokio;

use crate::model::image::Image;

#[tokio::main]
pub async fn run_image(image: &Image::Image) {
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
                Ok(info) => println!("{:?}", info),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Err(e) => eprintln!("Error: {}", e),

    }
}
