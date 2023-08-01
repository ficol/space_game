use clap::Parser;

/// Space Game Client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ip address to bind
    #[arg(long, default_value_t = String::from("127.0.0.1:8888"))]
    addr: String,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = client::run(&args.addr) {
        eprintln!("Client error: {e}");
        std::process::exit(1);
    }
}
