use shiplift::{Docker, PullOptions, RegistryAuth};
use futures::StreamExt;
use tokio;

use crate::model::image::Image;

#[tokio::main]
pub async fn pull_image(image: &Image::Image) {
    let docker = Docker::new();
    let auth = RegistryAuth::builder()
        .username(&image.username)
        .password(&image.password)
        .server_address(&image.host)
        .build();

    let mut stream = docker
        .images()
        .pull(&PullOptions::builder().image(&image.repository).auth(auth).build());

    while let Some(pull_result) = stream.next().await {
        match pull_result {
            Ok(output) => {
                let status = output["status"].as_str().unwrap();
                if status == "Downloading" {
                    println!("Downloading {:?}", output["progress"].as_str().unwrap())
                } else if status == "Extracting" {
                    println!("Extracting {:?}", output["progress"].as_str().unwrap())
                }
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
