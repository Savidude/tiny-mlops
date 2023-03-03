pub mod image {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Image {
        pub host: String,
        pub username: String,
        pub password: String,
        pub repository: String,
    }

    pub fn default () -> Image {
        Image{host: "registry.hub.docker.com".to_owned(), username: "".to_owned(), password: "".to_owned(), repository: "".to_owned()}
    }

    pub struct ArgExpose {
        pub srcport: u32,
        pub protocol: String,
        pub hostport: u32,
    }

    pub struct ImageArgs {
        pub expose: Option<ArgExpose>,
        pub volumes: Option<String>,
    }
}