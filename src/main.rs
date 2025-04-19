use clap::Parser;

use hyperwood::Variant;
use hyperwood_bench::{BenchParameters, build_model};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The width of the bench
    #[clap(long, required = true)]
    width: isize,

    /// The depth of the bench
    #[clap(long, required = true)]
    depth: isize,

    /// The height of the bench. Must be an integer, and greater than 3
    #[clap(long, required = true)]
    height: isize,

    /// Provide the slat variant to use. Specify each three dimension, eg 0.06x0.04x0.06
    #[clap(long, value_delimiter = 'x', required = true)]
    variant: Vec<f32>,
}

fn main() {
    let args = Args::parse();
    let parameters = BenchParameters::new(args.width, args.depth, args.height);
    let variant = Variant::new(args.variant[0], args.variant[1], args.variant[2]);
    let model = build_model(parameters, variant);

    print!("{}", model.to_hef());
}
