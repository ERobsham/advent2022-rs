

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
mod day11;

use std::fmt::Debug;

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
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
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
            Day::Day11 => day11::solve(input, part),
            Day::Day12 => unimplemented!(), // day12::solve(input, part),
            Day::Day13 => unimplemented!(), // day13::solve(input, part),
            Day::Day14 => unimplemented!(), // day14::solve(input, part),
            Day::Day15 => unimplemented!(), // day15::solve(input, part),
            Day::Day16 => unimplemented!(), // day16::solve(input, part),
            Day::Day17 => unimplemented!(), // day17::solve(input, part),
            Day::Day18 => unimplemented!(), // day18::solve(input, part),
            Day::Day19 => unimplemented!(), // day19::solve(input, part),
            Day::Day20 => unimplemented!(), // day20::solve(input, part),
            Day::Day21 => unimplemented!(), // day21::solve(input, part),
            Day::Day22 => unimplemented!(), // day22::solve(input, part),
            Day::Day23 => unimplemented!(), // day23::solve(input, part),
            Day::Day24 => unimplemented!(), // day24::solve(input, part),
            Day::Day25 => unimplemented!(), // day25::solve(input, part),
        }
    }    
}


#[derive(Clone, ValueEnum)]
pub enum Part {
    Part1 = 1,
    Part2,
}

impl Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Part1 => f.write_fmt(format_args!("Part-1")),
            Part::Part2 => f.write_fmt(format_args!("Part-2")),
        }
    }
}


#[cfg(test_output_bench)]
#[cfg(test)]
mod bench {
    use std::time::{Instant, Duration};
    use crate::*;
    
    fn test_input(n: u8) -> Option<Vec<String>> {
        use std::fs;

        let path = format!("input/day-{:02}", n);
        
        let input_file = fs::read_to_string(path.as_str()).ok()?;
        let lines:Vec<String> = input_file.split('\n')
        .map(|item| String::from(item)).collect();
    
        Some(lines)
    }

    fn run_bench(input: &Vec<String>, func: fn(Box<dyn Iterator<Item = String>>, Part)->String, part: Part) -> Duration {
        let test_iter = Box::new(input.clone().into_iter());
        let start = Instant::now();
        func(test_iter, part);
        Instant::now().duration_since(start)
    }

    #[test]
    fn combined_bench_test() {
        use crate::*;
        
        const NUM_RUNS:u32 = 100;

        let benches = vec![
            // (day, fn)
            (1, day01::solve as fn(Box<(dyn Iterator<Item = String> + 'static)>, Part) -> String),
            (2, day02::solve),
            (3, day03::solve),
            (4, day04::solve),
            (5, day05::solve),
            (6, day06::solve),
            (6, day06::solve),
            (7, day07::solve),
            (8, day08::solve),
            (9, day09::solve),
            (10, day10::solve),
            (11, day11::solve),
            // (12, day12::solve),
            // (13, day13::solve),
            // (14, day14::solve),
            // (15, day15::solve),
            // (16, day16::solve),
            // (17, day17::solve),
            // (18, day18::solve),
            // (19, day19::solve),
            // (20, day20::solve),
            // (21, day21::solve),
            // (22, day22::solve),
            // (23, day23::solve),
            // (24, day24::solve),
            // (25, day25::solve),
        ];

        println!("Running sudo benchmarks:  (averaging {} runs) ", NUM_RUNS);

        for (day_n, func) in benches {
            let input = test_input(day_n).unwrap();

            // part 1
            let mut total_time = Duration::default();
            for _ in 0..NUM_RUNS {
                total_time += run_bench(&input, func, Part::Part1);
            }
            print!("Day-{:02} {:?} duration: {:#?} \n", day_n, Part::Part1, total_time / NUM_RUNS);
            
            // part 2
            let mut total_time = Duration::default();
            for _ in 0..NUM_RUNS {
                total_time += run_bench(&input, func, Part::Part2);
            }

            print!("       {:?} duration: {:#?} \n\n", Part::Part2, total_time / NUM_RUNS);
        }

    }

}