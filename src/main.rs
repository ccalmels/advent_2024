use clap::Parser;
use std::env;
mod days;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "limit the number of threads used by rayon")]
    nthreads: Option<usize>,
    #[arg(
        short,
        long,
        help = "Advent Of Code session ID for automatic downloading of inputs"
    )]
    session: Option<String>,
    #[arg(trailing_var_arg = true)]
    days: Vec<u32>,
}

fn main() {
    let args = Args::parse();

    if let Some(nthreads) = args.nthreads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(nthreads)
            .build_global()
            .unwrap();
    }

    let session = args.session.or(env::var("AOC_SESSION").ok());

    advent_2024::resolve(session.as_deref(), &args.days);
}
