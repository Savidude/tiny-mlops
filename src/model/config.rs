pub mod Config {
    use serde::Deserialize;
    use serde::Serialize;
    use crate::model::image::image;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Config {
        pub fleet_id: String,
        pub model: image::Image
    }

    pub fn default () -> Config {
        Config{fleet_id: "".to_owned(), model: image::default()}
    }
}