

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

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
    Day8,
    Day9,
}
#[derive(Clone, ValueEnum)]
pub enum Part {
    Part1 = 1,
    Part2,
}


impl Solve for Day {
    fn solve(&self, input: Box<dyn Iterator<Item = String>>, part: Part) -> String {
        match self {
            Day::Day1 => day01::solve(input, part),
            Day::Day2 => day02::solve(input, part),
            Day::Day3 => day03::solve(input, part),
            Day::Day4 => day04::solve(input, part),
            Day::Day5 => day05::solve(input, part),
            Day::Day6 => day06::solve(input, part),
            Day::Day7 => day07::solve(input, part),
            Day::Day8 => day08::solve(input, part),
            Day::Day9 => day09::solve(input, part),
        }
    }    
}