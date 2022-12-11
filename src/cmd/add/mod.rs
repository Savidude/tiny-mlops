use std::io;
use std::io::Write;

#[derive(Debug)]
struct Client {
    client_host: String,
    client_username: String,
    client_passwd: String,
    client_repo: String,
}

impl Default for Client {
    fn default () -> Client {
        Client{client_host: "".to_owned(), client_username: "".to_owned(), client_passwd: "".to_owned(), client_repo: "".to_owned()}
    }
}

pub fn add_client(client_host: Option<String>, client_username: Option<String>, client_passwd: Option<String>, client_repo: Option<String>) {
    let mut client = Client::default(); 

    match client_host {
        Some(x) => {client.client_host = x},
        None    => {
            print!("Client registry host: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut client.client_host).expect("Didn't Receive Input");
            client.client_host = client.client_host.trim_end().to_owned()
        },
    }
    match client_username {
        Some(x) => {client.client_username = x},
        None    => {
            print!("Client registry username: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut client.client_username).expect("Didn't Receive Input");
            client.client_username = client.client_username.trim_end().to_owned()
        },
    }
    match client_passwd {
        Some(x) => {client.client_passwd = x},
        None    => {
            print!("Client registry password: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut client.client_passwd).expect("Didn't Receive Input");
            client.client_passwd = client.client_passwd.trim_end().to_owned()
        },
    }
    match client_repo {
        Some(x) => {client.client_repo = x},
        None    => {
            print!("Client repository: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut client.client_repo).expect("Didn't Receive Input");
            client.client_repo = client.client_repo.trim_end().to_owned()
        },
    }

    println!("{:?}", client);
}

#[derive(Debug)]
struct Monitor {
    monitor_endpoint: String,
}

impl Default for Monitor {
    fn default () -> Monitor {
        Monitor{monitor_endpoint: "".to_owned()}
    }
}

pub fn add_monitor(monitor_endpoint: Option<String>) {
    let mut monitor = Monitor::default();

    match monitor_endpoint {
        Some(x) => {monitor.monitor_endpoint = x},
        None    => {
            print!("Monitor endpoint: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut monitor.monitor_endpoint).expect("Didn't Receive Input");
            monitor.monitor_endpoint = monitor.monitor_endpoint.trim_end().to_owned()
        },
    }

    println!("{:?}", monitor);
}

#[derive(Debug)]
struct Service {
    service_protocol: String,
    service_port: i32,
}

impl Default for Service {
    fn default () -> Service {
        Service{service_protocol: "".to_owned(), service_port: 0}
    }
}

pub fn add_service(protocol: String, port: i32) {
    let mut service = Service::default();

    service.service_protocol = protocol;
    service.service_port = port;
    println!("{:?}", service);
}
