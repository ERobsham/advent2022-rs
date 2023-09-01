

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
            Day::Day11 => day11::solve(input, part),
        }
    }    
}

#[cfg(test_output_bench)]
#[cfg(test)]
mod bench {
    use std::{
        fs,
        time::Instant, 
    };
    use crate::Part;
    
    fn test_iter(n: u8) -> Option<Box<dyn Iterator<Item = String>>> {
        let path = format!("input/day-{:02}", n);
        
        let input_file = fs::read_to_string(path.as_str()).ok()?;
        let lines:Vec<String> = input_file.split('\n')
        .map(|item| String::from(item)).collect();
    let input = Box::new(lines.into_iter());
    
    Some(input)
    }

    #[test]
    fn test_day_01_pt1() {
        use crate::day01::solve;
        let iter = test_iter(1).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part1);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day01 Part1 duration: {:#?} \n", time_taken);
    }
    #[test]
    fn test_day_01_pt2() {
        use crate::day01::solve;
        let iter = test_iter(1).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part2);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day01 Part2 duration: {:#?} \n", time_taken);
    }

    #[test]
    fn test_day_02_pt1() {
        use crate::day02::solve;
        let iter = test_iter(2).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part1);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day02 Part1 duration: {:#?} \n", time_taken);
    }
    #[test]
    fn test_day_02_pt2() {
        use crate::day02::solve;
        let iter = test_iter(2).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part2);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day02 Part2 duration: {:#?} \n", time_taken);
    }

    #[test]
    fn test_day_03_pt1() {
        use crate::day03::solve;
        let iter = test_iter(3).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part1);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day03 Part1 duration: {:#?} \n", time_taken);
    }
    #[test]
    fn test_day_03_pt2() {
        use crate::day03::solve;
        let iter = test_iter(3).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part2);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day03 Part2 duration: {:#?} \n", time_taken);
    }

    #[test]
    fn test_day_04_pt1() {
        use crate::day04::solve;
        let iter = test_iter(4).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part1);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day04 Part1 duration: {:#?} \n", time_taken);
    }
    #[test]
    fn test_day_04_pt2() {
        use crate::day04::solve;
        let iter = test_iter(4).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part2);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day04 Part2 duration: {:#?} \n", time_taken);
    }

    #[test]
    fn test_day_05_pt1() {
        use crate::day05::solve;
        let iter = test_iter(5).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part1);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day05 Part1 duration: {:#?} \n", time_taken);
    }
    #[test]
    fn test_day_05_pt2() {
        use crate::day05::solve;
        let iter = test_iter(5).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part2);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day05 Part2 duration: {:#?} \n", time_taken);
    }

    #[test]
    fn test_day_06_pt1() {
        use crate::day06::solve;
        let iter = test_iter(6).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part1);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day06 Part1 duration: {:#?} \n", time_taken);
    }
    #[test]
    fn test_day_06_pt2() {
        use crate::day06::solve;
        let iter = test_iter(6).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part2);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day06 Part2 duration: {:#?} \n", time_taken);
    }

    #[test]
    fn test_day_07_pt1() {
        use crate::day07::solve;
        let iter = test_iter(7).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part1);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day07 Part1 duration: {:#?} \n", time_taken);
    }
    #[test]
    fn test_day_07_pt2() {
        use crate::day07::solve;
        let iter = test_iter(7).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part2);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day07 Part2 duration: {:#?} \n", time_taken);
    }

    #[test]
    fn test_day_08_pt1() {
        use crate::day08::solve;
        let iter = test_iter(8).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part1);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day08 Part1 duration: {:#?} \n", time_taken);
    }
    #[test]
    fn test_day_08_pt2() {
        use crate::day08::solve;
        let iter = test_iter(8).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part2);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day08 Part2 duration: {:#?} \n", time_taken);
    }

    #[test]
    fn test_day_09_pt1() {
        use crate::day09::solve;
        let iter = test_iter(9).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part1);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day09 Part1 duration: {:#?} \n", time_taken);
    }
    #[test]
    fn test_day_09_pt2() {
        use crate::day09::solve;
        let iter = test_iter(9).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part2);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day09 Part2 duration: {:#?} \n", time_taken);
    }

    #[test]
    fn test_day_10_pt1() {
        use crate::day10::solve;
        let iter = test_iter(10).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part1);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day10 Part1 duration: {:#?} \n", time_taken);
    }
    #[test]
    fn test_day_10_pt2() {
        use crate::day10::solve;
        let iter = test_iter(10).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part2);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day10 Part2 duration: {:#?} \n", time_taken);
    }

    #[test]
    fn test_day_11_pt1() {
        use crate::day11::solve;
        let iter = test_iter(11).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part1);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day11 Part1 duration: {:#?} \n", time_taken);
    }
    #[test]
    fn test_day_11_pt2() {
        use crate::day11::solve;
        let iter = test_iter(11).unwrap();
        let start = Instant::now();
        
        solve(iter, Part::Part2);
        
        let time_taken = Instant::now().duration_since(start);
        print!("Day11 Part2 duration: {:#?} \n", time_taken);
    }

}