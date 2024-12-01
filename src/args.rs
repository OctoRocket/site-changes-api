use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Port to serve the API on
    #[arg(short, long)]
    pub port: u16,
}