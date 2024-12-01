use clap::Parser;
mod days;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "limit the number of threads used by rayon")]
    nthreads: Option<usize>,
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

    advent_2024::resolve(&args.days);
}
