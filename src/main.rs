use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Name of path to analye
    #[arg(short, long)]
    check_db: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.check_db);
}
