use std::{
    io::{stdin, stdout, Result, Write}, 
    array::from_fn
};

// part 1
// const N_ELVES: usize = 1;

// part 2
const N_ELVES: usize = 3;

fn main() {

    let mut lines = stdin().lines();
    
    let total = get_top_n::<{N_ELVES+1}>(&mut lines);

    let mut out = stdout().lock();
    out.write_all(format!("{}\n", total).as_bytes())
        .expect("should be able to write to stdout");
}

/// make sure we pass in `N+1` otherwise, we'll constantly overwrite the `N`th value instead of pushing it out.
fn get_top_n<const N:usize>(lines: &mut dyn Iterator<Item=Result<String>>) -> usize {
    
    let mut top_n: [usize; N] = from_fn(|_i| 0);
    let mut curr_cals: usize = 0;

    while let Some(Ok(line)) = lines.next() {
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

    top_n.into_iter().fold(0, |acc, x| acc + x) - top_n[N-1]
}