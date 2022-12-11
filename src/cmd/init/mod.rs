use std::io;
use std::io::Write;

#[derive(Debug)]
struct Model {
    id: String,
    model_host: String,
    model_username: String,
    model_passwd: String,
    model_repo: String,
}

impl Default for Model {
    fn default () -> Model {
        Model{id: "".to_owned(), model_host: "".to_owned(), model_username: "".to_owned(), model_passwd: "".to_owned(), model_repo: "".to_owned()}
    }
}

pub fn initialize(id: Option<String>, model_host: Option<String>, model_username: Option<String>, model_passwd: Option<String>, model_repo: Option<String>) {
    let mut model = Model::default(); 

    match id {
        Some(x) => {model.id = x},
        None    => {
            print!("Fleet ID: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut model.id).expect("Didn't Receive Input");
            model.id = model.id.trim_end().to_owned()
        },
    }
    match model_host {
        Some(x) => {model.model_host = x},
        None    => {
            print!("Model registry host: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut model.model_host).expect("Didn't Receive Input");
            model.model_host = model.model_host.trim_end().to_owned()
        },
    }
    match model_username {
        Some(x) => {model.model_host = x},
        None    => {
            print!("Model registry username: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut model.model_username).expect("Didn't Receive Input");
            model.model_username = model.model_username.trim_end().to_owned()
        },
    }
    match model_passwd {
        Some(x) => {model.model_passwd = x},
        None    => {
            print!("Model registry password: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut model.model_passwd).expect("Didn't Receive Input");
            model.model_passwd = model.model_passwd.trim_end().to_owned()
        },
    }
    match model_repo {
        Some(x) => {model.model_repo = x},
        None    => {
            print!("Model repository: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut model.model_repo).expect("Didn't Receive Input");
            model.model_repo = model.model_repo.trim_end().to_owned()
        },
    }

    println!("{:?}", model);
}
