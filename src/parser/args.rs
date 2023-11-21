use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Template file path
    #[arg(short, long)]
    pub template: String,

}