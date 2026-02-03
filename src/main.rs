use clap::Parser;
use std::path::Path;

mod file_parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Name of path to analye
    #[arg(short, long)]
    check_db: String,
}

/// Print name of all installed packages
fn print_packages(path: &Path) {
    println!("Hello {:?}!", path.to_str());
    for entry in path.read_dir().expect("read_dir call failed").flatten() {
        println!("{:?}", entry.path());
        file_parser::get_pkg_name(entry.path().as_path());
    }
}

fn main() {
    let args = Args::parse();
    print_packages(Path::new(&args.check_db));
}
