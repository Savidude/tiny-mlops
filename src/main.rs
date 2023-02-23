use clap::{Args, Parser, Subcommand};

mod cmd;

mod model {
    pub mod image;
    pub mod config;
}
mod util {
    pub mod system;
    pub mod files;
    pub mod docker;
}
mod constants;
mod results;

use std::io::Write;
use env_logger::Builder;
use log::LevelFilter;
use std::env;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the edge device
    Init(Init),
    /// Add components to the edge device
    Add(Add),
    /// Manually pull images
    Pull(Pull),
}

#[derive(Args)]
struct Init {
    /// ID for the edge device
    #[arg(long)] 
    id: Option<String>,
    /// Hostname of the model container registry
    #[arg(long)]
    host: Option<String>,
    /// Username of the model container registry
    #[arg(long)]
    username: Option<String>,
    /// Password of the model container registry
    #[arg(long)]
    passwd: Option<String>,
    /// Model container repository
    #[arg(long)]
    repo: Option<String>,
}

#[derive(Args, Debug)]
struct Add {
    #[command(subcommand)]
    command: AddCommands,
}

#[derive(Subcommand, Debug)]
enum AddCommands {
    /// Initialize the edge device client
    Client(AddClient),
    /// Add monitoring to the device
    Monitor(AddMonitor),
    /// Add communication protocols
    Service(AddService),
}

#[derive(Args, Debug)]
struct AddClient {
    /// Hostname of the client container registry
    #[arg(long)]
    host: Option<String>,
    /// Username of the client container registry
    #[arg(long)]
    username: Option<String>,
    /// Password of the client container registry
    #[arg(long)]
    passwd: Option<String>,
    /// Client container repository
    #[arg(long)]
    repo: Option<String>,
}

#[derive(Args, Debug)]
struct AddMonitor {
    /// Monitor endpoint
    #[arg(long)]
    endpoint: Option<String>,
}

#[derive(Args, Debug)]
struct AddService {
    /// Service protocol (HTTP, MQTT)
    #[arg(long)]
    protocol: String,
    /// Service port
    #[arg(long)]
    port: i32,
}

#[derive(Args, Debug)]
struct Pull {
    #[command(subcommand)]
    command: PullCommands,
}

#[derive(Subcommand, Debug)]
enum PullCommands {
    /// Pull model image
    Model(PullModel),
    /// Pull client image
    Client(PullClient),
}

#[derive(Args, Debug)]
struct PullModel {
}

#[derive(Args, Debug)]
struct PullClient {
}

fn enable_logging() {
    let mut builder = Builder::new();
    // builder.format_timestamp(None).format_module_path(false);
    builder.format_timestamp(None);
    builder.format(|buf, record| {
        writeln!(buf, "{} [{}] {}", buf.timestamp(), record.level(), record.args())
    });
    if let Ok(level) = env::var("EDGEOPS_LOG_LEVEL") {
        builder.filter_level(level.parse().unwrap_or(LevelFilter::Info));
    } else {
        builder.filter_level(LevelFilter::Info);
    }
    builder.init();
}

fn main() {
    enable_logging();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(init) => {
            cmd::init::initialize(init.id.clone(), init.host.clone(), init.username.clone(), init.passwd.clone(), init.repo.clone());
        }
        Commands::Add(add) => {
            match &add.command {
                AddCommands::Client(client) => {
                    cmd::add::add_client(client.host.clone(), client.username.clone(), client.passwd.clone(), client.repo.clone())
                }
                AddCommands::Monitor(monitor) => {
                    cmd::add::add_monitor(monitor.endpoint.clone())
                }
                AddCommands::Service(service) => {
                    cmd::add::add_service(service.protocol.clone(), service.port)
                }
            }
        }
        Commands::Pull(pull) => {
            match &pull.command {
                PullCommands::Model(model) => {
                    cmd::pull::pull_model()
                }
                PullCommands::Client(client) => {
                    cmd::pull::pull_client()
                }
            }
        }
    }
}
