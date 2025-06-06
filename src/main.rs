use clap::Parser;

fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
#[derive(Debug,Parser)]
#[command(version,about,long_about)]
struct Args {

    #[arg(short,long)]
    name: String,

    #[arg(short,long,default_value_t = 1)]
    count: i32,
}
