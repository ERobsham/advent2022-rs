use clap::Parser;
use std::{
    path::PathBuf, 
    io::{
        BufReader, 
        BufRead,
        stdin, stdout, Write
    }, 
    fs::File
};

use advent::{Day, Part, Solve};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_enum, default_value_t = Day::Day1)]
    day: Day,
    #[arg(value_enum, default_value_t = Part::Part1)]
    part: Part,
    
    /// optional path to specify the input file to use.
    #[arg(short,long)]
    input: Option<PathBuf>,
}


fn main() {
    let args = Cli::parse();

    let buf = 
        get_input_file(args.input)
        .unwrap_or(get_stdinput());

    let answer = args.day.solve(buf, args.part);

    let _ = stdout().write_all(format!("{}\n", answer).as_bytes());
}   

fn get_input_file(path: Option<PathBuf>) -> Option<Box<dyn Iterator<Item = String>>> {

    let path = path?;
    Some(
        Box::new(
            BufReader::new(File::open(path).ok()?)
            .lines()
            .filter_map(|l| l.ok() )
        )
    )
}

fn get_stdinput() -> Box<dyn Iterator<Item = String>> {
    Box::new(
        stdin().lines().filter_map(|l| l.ok() )
    )
}