
use crate::Part;


pub(crate) fn solve(input: Box<dyn Iterator<Item = String>>, part: Part) -> String {
    
    let forrest: Vec<Vec<i8>> = 
        input.map(|s| s.chars()
                .filter_map(|c| c.to_digit(10))
                .filter_map(|d| i8::try_from(d).ok())
                .collect())
            .collect();

    let total = match part {
        Part::Part1 => count_visible_trees(forrest),
        Part::Part2 => calc_max_scenic_value(forrest),
    };
    format!("{}", total)
}

// part 1 calculations
fn count_visible_trees(forrest: Vec<Vec<i8>>) -> usize {
    
    let height = forrest.len();
    if height < 1 { return 0; }
    let width = forrest.first().unwrap().len();

    let mut visable_tree_flags: Vec<Vec<i8>> = vec![vec![0_i8; width]; height];
    let mut max_col_heights = vec![-1_i8; width];

    // walk the rows from top->bottom & left->right
    let mut rows = forrest.iter().enumerate();
    while let Some((i, row)) = rows.next() {
        let mut tree_heights = row.iter().enumerate();
        let mut max_row_height = -1_i8;

        while let Some((j, tree_height)) = tree_heights.next() {
            if tree_height > &max_row_height {
                visable_tree_flags[i][j] = 1;
                max_row_height = tree_height.clone();
            }
            
            if tree_height > &max_col_heights[j] {
                visable_tree_flags[i][j] = 1;
                max_col_heights[j] = tree_height.clone();
            }
        }
    }

    // reset max column hights
    let mut max_col_heights = vec![-1_i8; width];

    // reverse the visible flags lists:
    let mut visable_tree_flags: Vec<Vec<i8>> = 
        visable_tree_flags.iter().rev()
            .map(|row| 
                row.iter().rev().cloned().collect())
            .collect();

    // walk the rows from bottom->top & right->left
    let mut rows = forrest.iter().rev().enumerate();
    while let Some((i, row)) = rows.next() {
        let mut tree_heights = row.iter().rev().enumerate();
        let mut max_row_height = -1_i8;

        while let Some((j, tree_height)) = tree_heights.next() {
            if tree_height > &max_row_height {
                visable_tree_flags[i][j] = 1;
                max_row_height = tree_height.clone();
            }
            
            if tree_height > &max_col_heights[j] {
                visable_tree_flags[i][j] = 1;
                max_col_heights[j] = tree_height.clone();
            }
        }
    }


    visable_tree_flags.iter().fold(0_usize, |acc_total, row| {
        acc_total +
        row.iter().fold(0_usize, |acc_row, v| {
            acc_row + v.to_owned().try_into().unwrap_or(0) 
        })
    })
}

// part 2 calculations
fn calc_max_scenic_value(forrest: Vec<Vec<i8>>) -> usize {
    
    let height = forrest.len();
    if height < 1 { return 0; }
    let width = forrest.first().unwrap().len();

    let mut tree_scenic_values: Vec<Vec<usize>> = vec![vec![1_usize; width]; height];

    for i in 0..height {
        for j in 0..width {
            tree_scenic_values[i][j] = 
                calc_scenic_value_for((i,j), &forrest);
        }
    }

    tree_scenic_values.iter()
        .map(|row| row.iter().max().cloned().unwrap_or(0))
        .max().unwrap_or(0)
}

fn calc_scenic_value_for((i,j): (usize, usize), forrest: &Vec<Vec<i8>>) -> usize {
    
    let height = forrest.len();
    if height < 1 { return 1; }
    let width = forrest.first().unwrap().len();
    
    if i > height || 
        j > width {
        return 1;
    }

    let our_tree_height = forrest.get(i).unwrap().get(j).unwrap();

    let our_row = forrest.get(i).unwrap().clone();
    let row_slice = our_row.as_slice();
    let row_start = &row_slice[..j];
    let row_end = &row_slice[j..];
    
    let our_col: Vec<i8> = forrest.iter().map(|row| row.get(j).unwrap().clone()).collect();
    let col_slice = our_col.as_slice();
    let col_start = &col_slice[..i];
    let col_end = &col_slice[i..];
    
    // row view distances:
    let mut left_view_dist = 0_usize;
    let left = row_start.iter().rev();
    for v in left {
        left_view_dist += 1;
        if v >= our_tree_height {
            break;
        }
    }

    let mut right_view_dist = 0_usize;
    let mut right = row_end.iter();
    let _ = right.next();
    for v in right {
        right_view_dist += 1;
        if v >= our_tree_height {
            break;
        }
    }


    // col view distance:
    let mut up_view_dist = 0_usize;
    let up = col_start.iter().rev();
    for v in up {
        up_view_dist += 1;
        if v >= our_tree_height {
            break;
        }
    }

    let mut down_view_dist = 0_usize;
    let mut down = col_end.iter();
    let _ = down.next();
    for v in down {
        down_view_dist += 1;
        if v >= our_tree_height {
            break;
        }
    }

    up_view_dist * left_view_dist * down_view_dist * right_view_dist
}


#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"30373
25512
65332
33549
35390";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));

    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "21");
    
    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "8");
}


#[cfg(test_output)]
#[test]
fn test_input_file() {
    const EXAMPLE: &str = r"input/day-8";

    let input_file = std::fs::read_to_string(EXAMPLE).unwrap();
    let lines:Vec<String> = input_file.split('\n')
        .map(|item| String::from(item)).collect();
    let input = Box::new(lines.into_iter());
        
    let output = solve(input.clone(), Part::Part1);
    println!("{}", output);
    // assert_eq!(output.as_str(), "21");
    
    let output = solve(input, Part::Part2);
    println!("{}", output);
    // assert_eq!(output.as_str(), "8");
}
