
use crate::Part;

const ROUNDS_P1:usize = 20;
const ROUNDS_P2:usize = 10000;

const TOP_N:usize = 2;

#[derive(Default)]
enum Operand {
    #[default]
    Add,
    Sub,
    Multi,
    Divide,
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Multi,
            "/" => Self::Divide,
            _ => Self::default(),
        }
    }
}

impl Operand {
    fn do_op(&self, left:u64, right:u64) -> u64 {
        match *self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Multi => left * right,
            Self::Divide => left / right,
        }
    }
}

#[derive(Default)]
enum Value {
    #[default]
    Variable,
    Constant(u64),
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        let val = value.parse::<u64>();
        match val {
            Ok(x) => Self::Constant(x),
            Err(_) => Self::Variable,
        }
    }
}

impl Value {
    fn const_or(&self, var: u64) -> u64 {
        match *self {
            Self::Constant(x) => x,
             Self::Variable => var,
        }
    }
}

#[derive(Default)]
struct  Expression {
    left: Value,
    right: Value,
    op: Operand,
}

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        let (_, exp) = value.split_once("=")
            .expect("expressions have an '='");

        let mut chunks = exp.trim().split(" ");

        let left = chunks.next().expect("need left side of expression").into();
        let op = chunks.next().expect("need op of expression").into();
        let right = chunks.next().expect("need left side of expression").into();

        Self { left, right, op }
    }
}

impl Expression {
    fn calc_with_var(&self, x: u64) -> u64 {
        self.op.do_op(
            self.left.const_or(x),
            self.right.const_or(x)
        )
    }
}

type MonkeyIndex = usize;

struct Monkey {
    total_inspections: usize,
    items: Vec<u64>,
    operation: Expression,
    test_modulus: u64,
    option1: MonkeyIndex,
    option2: MonkeyIndex,
}

impl From<Vec<String>> for Monkey {
    fn from(lines: Vec<String>) -> Self {
        let mut items = Vec::default();
        let mut operation = Expression::default();
        let mut test_modulus = u64::default();
        let mut option1: MonkeyIndex = usize::default();
        let mut option2: MonkeyIndex = usize::default();

        let mut lines = lines.iter();
        while let Some(line) = lines.next() {
            let (left,right) = line.trim().split_once(":")
                .expect("all lines have :");
            
            match left {
                _ if left.starts_with("Starting items") => {
                    items = right.split(",")
                        .filter_map(|c| c.trim().parse().ok())
                        .collect();
                },
                _ if left.starts_with("Operation") => operation = right.into(),
                _ if left.starts_with("Test") => {
                    let (_, right) = right
                        .split_once(" by ")
                        .expect("invalid 'Test' format");
                    test_modulus = right.trim().parse().expect("'Test' should be usize");
                },
                _ if left.starts_with("If true") => {
                    let (_, right) = right
                        .split_once(" monkey ")
                        .expect("invalid 'If' format");
                    option1 = right.trim().parse().expect("'If' should be usize");
                },
                _ if left.starts_with("If false") => {
                    let (_, right) = right
                        .split_once(" monkey ")
                        .expect("invalid 'If' format");
                    option2 = right.trim().parse().expect("'If' should be usize");
                },
                _ => {},
            }
        }

        Monkey { total_inspections: 0, items, operation, test_modulus, option1, option2 }
    }
}

impl Monkey {
    fn do_monkey_business(&mut self, do_calming: bool) -> Vec<(u64, MonkeyIndex)> {
        let mut num_inspections = 0_usize;
        let mut thrown_items = Vec::default();

        while let Some(item) = self.items.pop() {
            // inspect (apply worry level change operation)
            let mut new_item = self.operation.calc_with_var(item);
            num_inspections += 1;
            
            if do_calming {
                new_item /= 3;
            }

            // test
            let monkey_index = if new_item % self.test_modulus == 0 {
                self.option1
            } else {
                self.option2
            };

            thrown_items.push((new_item, monkey_index))
        }
        self.total_inspections += num_inspections;

        thrown_items
    }
}

fn chunker(input: &mut Box<dyn Iterator<Item = String>>, delimiter: &str) -> Option<Vec<String>> {
    let mut lines:Vec<String> = Vec::new();

    while let Some(line) = input.next() {
        if line.as_str() == delimiter {
            break;
        }
        
        lines.push(line);
    }

    if lines.len() == 0 {
        None
    } else {
        Some(lines)
    }
}

pub(crate) fn solve(mut input: Box<dyn Iterator<Item = String>>, part: Part) -> String {

    let mut monkey_list:Vec<Monkey> = Vec::new();

    while let Some(monkey_info) = chunker(&mut input, "")  {
        monkey_list.push(monkey_info.into());
    }

    let total = match part {
        Part::Part1 => calc_top_n_monkey_business(monkey_list, ROUNDS_P1, true),
        Part::Part2 => calc_top_n_monkey_business(monkey_list, ROUNDS_P2, false),
    };

    format!("{}", total)
}

fn calc_top_n_monkey_business(mut monkey_list: Vec<Monkey>, num_rounds: usize, do_calming: bool) -> u64 {

    // calc LCM
    let lcm = monkey_list.iter()
        .map(|m| m.test_modulus)
        .fold(1, |acc, x| acc * x);

    // run the sim
    for _round in 0..num_rounds {
        
        for idx in 0..monkey_list.len() {
            let m = monkey_list.get_mut(idx)
                .expect("already checked bounds");
            let thrown  = m.do_monkey_business(do_calming);

            for (item, idx) in thrown {

                let adj_item = if item > lcm { item % lcm } else { item };

                if let Some(other_monkey) = monkey_list.get_mut(idx) {
                    other_monkey.items.push(adj_item);
                }
            }
        }

    }

    // calc the top n
    monkey_list.sort_by(|a,b| b.total_inspections.cmp(&a.total_inspections));
    let _discard_end = monkey_list.split_off(TOP_N);

    // return the product of the top monkeys total inspections
    monkey_list.iter()
        .fold(1_u64, |acc, m| acc * m.total_inspections as u64)
}


#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));

    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "10605");
    
    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "2713310158");
}
