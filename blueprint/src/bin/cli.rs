use clap::{Parser, Subcommand};
use log::{info, warn};
use std::path::PathBuf;

use blueprintlib::infra::log::init as init_log;
use blueprintlib::modules::blueprint::{process, document::{DocumentParams}};

#[derive(Subcommand)]
enum Commands {
  /// Create svelte documentation files
  Document {
    /// Sets a source path
    #[arg(short, long, value_name = "FILE")]
    source: PathBuf,

    /// Sets a destination path
    #[arg(short, long, value_name = "Directory")]
    destination: PathBuf,

    /// Sets a template for the final component
    #[arg(short, long, value_name = "FILE")]
    template: Option<PathBuf>,
  },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
  /// Turn debugging information on
  #[arg(short, long, action = clap::ArgAction::Count)]
  verbose: u8,

  #[command(subcommand)]
  command: Option<Commands>,
}

fn main() {
  let cli = Cli::parse();
  init_log(cli.verbose);

  match &cli.command {
    Some(Commands::Document { source, destination, template }) => {
      if let Some(pb) = template {
        warn!("Using template: {}", pb.to_str().unwrap());
      }

      info!("Destination directory: {}", destination.to_str().unwrap());

      let params = DocumentParams { destination, template };

      process(source, params);

  },
    None => {
      info!("No action performed")
    }
  }

  
}
