use clap::Parser;

/// Space Game Client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to json file with map configuration
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    ip: String,

    /// Port to bind server
    #[arg(long, default_value_t = 8888)]
    port: u32,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = client::run(&args.ip, args.port) {
        eprintln!("Client error: {e}");
        std::process::exit(1);
    }
}
