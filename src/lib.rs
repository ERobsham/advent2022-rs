

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use clap::ValueEnum;

/// the main trait each 'day' module should implement to solve that day's input
pub trait Solve {
    fn solve(&self, input: Box<dyn Iterator<Item = String>>, part: Part) -> String;
}

#[derive(Clone, ValueEnum)]
pub enum Day {
    Day1 = 1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
}
#[derive(Clone, ValueEnum)]
pub enum Part {
    Part1 = 1,
    Part2,
}


impl Solve for Day {
    fn solve(&self, input: Box<dyn Iterator<Item = String>>, part: Part) -> String {
        match self {
            Day::Day1 => day1::solve(input, part),
            Day::Day2 => day2::solve(input, part),
            Day::Day3 => day3::solve(input, part),
            Day::Day4 => day4::solve(input, part),
            Day::Day5 => day5::solve(input, part),
            Day::Day6 => day6::solve(input, part),
            Day::Day7 => day7::solve(input, part),
        }
    }    
}