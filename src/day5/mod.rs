use std::collections::HashMap;

use crate::Part;

type StackID = char;
type CrateID = char;

enum CraneType {
    CrateMover9000,
    CrateMover9001,
}

struct CrateState {
    // stack id => 'crates' in the stack
    crate_stacks: HashMap<StackID, Vec<CrateID>>,
}

impl From<Vec<String>> for CrateState {
    fn from(mut header_lines: Vec<String>) -> Self {
        
        let mut stack_idx_map:HashMap<StackID, usize> = HashMap::new();
        if let Some(header) = header_lines.pop() {
            let char_idxes = header.char_indices();
            // parse columns

            char_idxes.for_each(|(i, c)| {
                let _ = c.is_digit(10) && 
                // we only want to run this if 'c' is a digit, 
                // and this should allow this to compile as branchless
                stack_idx_map.insert(c, i).is_none();
            });
        }
        // drop mutability
        let stack_idx_map = stack_idx_map;

        let mut crate_stacks: HashMap<StackID, Vec<CrateID>> = HashMap::new();
        while let Some(line) = header_lines.pop() {
            let line_chars: Vec<char> = line.chars().collect();
            // parse the crates in each column

            stack_idx_map.iter().for_each(|(id,idx)| {
                let crate_id = line_chars.get(idx.to_owned())
                    .expect("'idx' must map to a value in the preceding lines")
                    .to_owned();

                if crate_id.is_whitespace() { return; }

                if !crate_stacks.contains_key(id) {
                    crate_stacks.insert(id.to_owned(), Vec::new());
                }
                let stack = crate_stacks.get_mut(id).expect("already checked");
                stack.push(crate_id);
            });
        }

        CrateState { crate_stacks }
    }

}

impl CrateState {
    fn apply(&mut self, cmd: Command) {
        if cmd.from == cmd.to ||
            self.crate_stacks.get(&cmd.from).is_none() || 
            self.crate_stacks.get(&cmd.to).is_none()  {
            return;
        }

        let src = self.crate_stacks.get_mut(&cmd.from).expect("already checked");
        let mut stack_to_move: Vec<CrateID> = Vec::new();
        for _ in 0..cmd.num {
            if let Some(c) = src.pop() {
                stack_to_move.push(c);
            }
        }

        if let CraneType::CrateMover9001 = cmd.version {
            stack_to_move.reverse();
        }

        let dest = self.crate_stacks.get_mut(&cmd.to).expect("already checked");
        dest.extend(stack_to_move.iter());
    }

    fn top_crates(&self) -> String {
        let mut result = String::new();

        let mut keys: Vec<StackID> = self.crate_stacks.keys()
            .cloned()
            .collect();
        keys.sort_by(|a,b| {
            let a = a.to_digit(10).expect("stackIDs should be digits!");
            let b = b.to_digit(10).expect("stackIDs should be digits!");
            a.cmp(&b)
        });

        let top_crates: Vec<char> = keys.iter().filter_map(|k| {
            let top_crate = self.crate_stacks
                .get(k)?
                .last()?;

            Some(top_crate.to_owned())
        }).collect();

        result.extend(top_crates.iter());

        result
    }
}


struct Command {
    num: usize,
    from: StackID,
    to: StackID,
    version: CraneType,
}

impl Command {
    fn as_9000(mut self) -> Self {
        self.version = CraneType::CrateMover9000;
        self
    }
    
    fn as_9001(mut self) -> Self {
        self.version = CraneType::CrateMover9001;
        self
    }
}

impl From<String> for Command {
    fn from(line: String) -> Self {
        let mut num: usize = 0;
        let mut from: StackID = '0';
        let mut to: StackID = '0';

        let mut components = line.split(' ');
        for _ in 0..3 {
            if let (Some(op), Some(val)) = (components.next(), components.next()) {
                match (op, val) {
                    ("move", num_str) => num = num_str.parse().unwrap_or_default(),
                    ("from", from_str) => from = from_str.chars().next().unwrap_or('0'),
                    ("to", to_str) => to = to_str.chars().next().unwrap_or('0'),
                    _ => unimplemented!("unknown command syntax"),
                }
            }
        }

        return Command { num, from, to, version: CraneType::CrateMover9000 }
    }
}



pub(crate) fn solve(mut input: Box<dyn Iterator<Item = String>>, part: Part) -> String {
    
    // step 1: parse header to get initial state of crate stacks:
    let mut header_lines: Vec<String> = Vec::new();
    while let Some(line) = input.next() {
        if line.is_empty() {
            break;
        }
        header_lines.push(line);
    }

    let mut crates: CrateState = header_lines.into();

    // step 2: parse & apply each 'command'
    while let Some(line) = input.next() {
        let cmd: Command = line.into();

        match part {
            Part::Part1 => crates.apply(cmd.as_9000()),
            Part::Part2 => crates.apply(cmd.as_9001()),
        }
    }

    // finally, just grab the 'top crate' for every stack
    crates.top_crates()
}



#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));


    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "CMZ");
    
    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "MCD");
}