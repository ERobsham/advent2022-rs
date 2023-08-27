use std::array::from_fn;
use crate::Part;

const N_ELVES_PT1: usize = 1;
const N_ELVES_PT2: usize = 3;

pub(crate) fn solve(input: Box<dyn Iterator<Item = String>>, part: Part) -> String {
    
    let total = match part {
        Part::Part1 => get_top_n::<{N_ELVES_PT1+1}>(input),
        Part::Part2 => get_top_n::<{N_ELVES_PT2+1}>(input),
    };
    format!("{}", total)
}

/// make sure we pass in `N+1` otherwise, we'll constantly overwrite the `N`th value instead of pushing it out.
fn get_top_n<const N:usize>(mut lines: Box<dyn Iterator<Item=String>>) -> usize {
    
    let mut top_n: [usize; N] = from_fn(|_i| 0);
    let mut curr_cals: usize = 0;

    while let Some(line) = lines.next() {
        if line.is_empty() {
            top_n[N-1] = curr_cals;
            top_n.sort_by(|a,b| a.cmp(b).reverse());

            curr_cals = 0;
            continue;
        }

        if let Ok(cal) = line.parse::<usize>() {
            curr_cals += cal
        }
    }

    // add the last group if needed, just in case 
    // theres not a newline at the end of the last group.
    if curr_cals > 0 {
        top_n[N-1] = curr_cals;
        top_n.sort_by(|a,b| a.cmp(b).reverse());
    }

    top_n.into_iter().fold(0, |acc, x| acc + x) - top_n[N-1]
}



#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));

    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "24000");
    
    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "45000");
}
