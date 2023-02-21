use std::env;

pub fn get_home_dir() -> &str {
    match env::var("HOME") {
        Ok(home) => return home,
        Err(_) => {
            println!("Couldn't find home directory. Please set a value for the HOME environment variable.")
            return ""
        },
    }
}