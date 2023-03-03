use std::env;
use crate::constants;

fn get_home_dir() -> String {
    match env::var("HOME") {
        Ok(home) => return home,
        Err(_) => {
            println!("Couldn't find home directory. Please set a value for the HOME environment variable.");
            return String::from("");
        },
    }
}

pub fn get_edge_home_dir() -> String {
    let home_dir = get_home_dir();
    let edge_home = &format!("{}/{}", home_dir, constants::EDGE_HOME_DIR);
    return edge_home.to_string()
}
