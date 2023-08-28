

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

use clap::ValueEnum;

/// the main trait each 'day' module should implement to solve that day's input
pub trait Solve {
    fn solve(&self, input: Box<dyn Iterator<Item = String>>, part: Part) -> String;
}

#[derive(Clone, ValueEnum)]
pub enum Day {
    Day01 = 1,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
}
#[derive(Clone, ValueEnum)]
pub enum Part {
    Part1 = 1,
    Part2,
}


impl Solve for Day {
    fn solve(&self, input: Box<dyn Iterator<Item = String>>, part: Part) -> String {
        match self {
            Day::Day01 => day01::solve(input, part),
            Day::Day02 => day02::solve(input, part),
            Day::Day03 => day03::solve(input, part),
            Day::Day04 => day04::solve(input, part),
            Day::Day05 => day05::solve(input, part),
            Day::Day06 => day06::solve(input, part),
            Day::Day07 => day07::solve(input, part),
            Day::Day08 => day08::solve(input, part),
            Day::Day09 => day09::solve(input, part),
            Day::Day10 => day10::solve(input, part),
        }
    }    
}