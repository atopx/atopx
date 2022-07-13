use crate::flow::Flow;
use crate::osp::Osp;
use clap::{Parser, Subcommand};
use std::env;

mod flow;
mod osp;

#[derive(Subcommand)]
enum Commands {
    /// opeation related flow.
    Flow {
        /// label in [qa, ]
        #[clap(value_parser)]
        label: String,
    },

    /// os related commands.
    OS {

        /// lsof -i tcp:[port]
        #[clap(long, value_parser)]
        fpo: Option<u16>,

        /// ps -ef | grep [process]
        #[clap(long, value_parser)]
        fps: Option<String>
    }
}


/// atopx private cli tools.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// atopx subcommands
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Flow { label } => {
            let flower = Flow::new(env::current_dir().unwrap().to_str().expect("get current dir error."));
            match label.as_str() {
                "qa" => {
                    flower.qa();
                }
                _ => {
                    println!("unknown flow label: {}, please run atopx --help", label)
                }
            }
            
        }
        Commands::OS { fpo, fps } => {
            let osp = Osp::new(env::current_dir().unwrap().to_str().expect("get current dir error."));
            if let Some(value) = fpo {
                osp.lsof_tcp(value);
            }
            if let Some(value) = fps {
                osp.ps_grep(value);
            }
        }
    }
}
