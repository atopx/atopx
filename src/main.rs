use crate::flow::Flow;
use clap::Parser;

mod flow;

/// StarUnion related tools.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Run related commoands of code.
    #[clap(short, long, value_parser)]
    flow: String,
}

fn main() {
    let args = Args::parse();
    if args.flow == "qa" {
        let flow = Flow::new("/Users/atopx/workspace/star_union_advertiser_api");
        flow.qa();
    } else {
        println!("unknown command, please run [./staru --help].");
    }
}
