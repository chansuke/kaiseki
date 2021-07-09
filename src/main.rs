use std::path::PathBuf;
use structopt::StructOpt;

use kaiseki::errors::StatsError;
use kaiseki::stats::get_stats_for_dir;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "kaiseki",
    about = "This is a tool to generate statistics on Rust projects"
)]
struct Opt {
    #[structopt(name = "source directory", parse(from_os_str))]
    in_dir: PathBuf,
    #[structopt(name = "mode", short)]
    mode: String,
}

fn main() -> Result<(), StatsError> {
    let opt = Opt::from_args();
    let mode = &opt.mode[..];

    match mode {
        "src" => {
            let stats = get_stats_for_dir(&opt.in_dir)?;

            println!("Result: {:?}", stats);
        }
        _ => println!("Sorry, no statistics"),
    }

    Ok(())
}
