use std::process::exit;
use clap::Parser;
use robot_simulator::*;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    x: i32,
    #[arg(short, long)]
    y: i32,
    #[arg(short, long)]
    d: String,
    #[arg(short, long)]
    seq: String
}

fn main() {
    let args = Args::parse();
    let robot = Robot::new(args.x, args.y, translate_dir(args.seq.as_str()));
    let _ = robot.instructions(args.seq.as_str());
}

fn translate_dir(seq: &str) -> Direction {
    match seq {
        "N"=>Direction::North,
        "S"=>Direction::South,
        "E"=>Direction::East,
        "W"=>Direction::West,
        _ => {
            println!("DIRECTION INSERTED NOT VALID: {}", seq);
            exit(1);
        }
    }
}