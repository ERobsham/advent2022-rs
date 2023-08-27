use std::fmt::Display;

use crate::Part;

const LOWERCASE_OFFSET: u8 = 96;
const UPPERCASE_OFFSET: u8 = 38;

struct ItemFlag {
    pub flag: u64,
}

impl ItemFlag {
    // add a byte that represents the item as a character (a-z,A-Z)
    fn add(&mut self, item: &u8) {
        let offset:u8;
        match item {
            97..=122 => offset = item - LOWERCASE_OFFSET,
            65..=90 => offset = item - UPPERCASE_OFFSET,
            _ => return,
        }

        self.flag |= 1 << (offset - 1);
    }

    // should get us the only shared result between the two sets of bits.
    fn intersect(&self, other: &ItemFlag) -> ItemFlag {
        ItemFlag { flag: self.flag & other.flag }
    }

    fn as_priorities(&self) -> Vec<u8> {

        let bytes:Vec<u8> = (0u8..52).filter_map(|i| {
            let bit: u64 = 1 << i;
            if self.flag & bit == 0 {
                None
            } else {
                Some(i + 1)
            }
        }).collect();

        bytes
    }

    fn priority_value(&self) -> usize {
        self.as_priorities().iter().fold(0, |acc, i| acc+usize::from(i.to_owned()))
    }
}

impl Display for ItemFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MASK_26: u64 = 0b111111_1111111111_1111111111;

        let lower = self.flag & MASK_26;
        let upper = (self.flag >> 26) & MASK_26;
        f.write_fmt(format_args!("({:026b} {:026b})", upper, lower))
    }
}

pub(crate) fn solve(input: Box<dyn Iterator<Item = String>>, part: Part) -> String {

    let total = match part {
        Part::Part1 => calc_total_priorities_pt1(input),
        Part::Part2 => calc_total_priorities_pt2(input),
    };

    format!("{}", total)
}

fn calc_total_priorities_pt1(mut lines: Box<dyn Iterator<Item = String>>) -> usize {

    let mut total_score: usize = 0;
    
    while let Some(line) = lines.next() {
        let split_idx = line.len() / 2;
        let (left, right) = line.split_at(split_idx);

        let mut part1 = ItemFlag { flag:0 };
        let mut part2 = ItemFlag { flag:0 };

        left.as_bytes().iter().for_each(|item | part1.add(item));
        right.as_bytes().iter().for_each(|item | part2.add(item));
        
        let result = part1.intersect(&part2);

        total_score += result.priority_value();
    }

    total_score
}
fn calc_total_priorities_pt2(mut lines: Box<dyn Iterator<Item = String>>) -> usize {

    let mut total_score: usize = 0;
    
    let mut idx: usize = 0;
    let mut group = (ItemFlag{flag:0}, ItemFlag{flag:0}, ItemFlag{flag:0});

    while let Some(line) = lines.next() {
        let mut pack = ItemFlag { flag:0 };
        line.as_bytes().iter().for_each(|item | pack.add(item));
        
        match idx % 3 {
            0 => {
                if idx > 0 {
                    total_score += calc_group_priorities(&group);

                    group.0.flag = 0;
                    group.1.flag = 0;
                    group.2.flag = 0;
                }

                group.0.flag = pack.flag;
            },
            1 => group.1.flag = pack.flag,
            2 => group.2.flag = pack.flag,
            _ => unreachable!(),
        }

        idx += 1;
    }
    total_score += calc_group_priorities(&group);

    total_score
}

fn calc_group_priorities(group: &(ItemFlag, ItemFlag, ItemFlag) ) -> usize {
    let result = group.0
                        .intersect(&group.1)
                        .intersect(&group.2);

    result.priority_value()
}


#[test] 
// quickly identify the values we'll need to map from.
fn test_utf8_vals() {
    let a = "a".as_bytes()[0];
    let z = "z".as_bytes()[0];
    let cap_a = "A".as_bytes()[0];
    let cap_z = "Z".as_bytes()[0];

    // print!("a:{} - z:{} | A:{} - Z:{} \n", a, z, cap_a, cap_z);

    assert_eq!(a - u8::try_from(LOWERCASE_OFFSET).unwrap(), 1);
    assert_eq!(z - u8::try_from(LOWERCASE_OFFSET).unwrap(), 26);

    assert_eq!(cap_a - u8::try_from(UPPERCASE_OFFSET).unwrap(), 27);
    assert_eq!(cap_z - u8::try_from(UPPERCASE_OFFSET).unwrap(), 52);
}

#[test]
// quick visualization of the bits flagged for each character parsed.
fn test_flags() {
    
    let a = "a".as_bytes()[0];
    let b = "b".as_bytes()[0];
    let c = "c".as_bytes()[0];
    let z = "z".as_bytes()[0];
    
    
    let cap_a = "A".as_bytes()[0];
    let cap_x = "X".as_bytes()[0];
    let cap_y = "Y".as_bytes()[0];
    let cap_z = "Z".as_bytes()[0];
    
    let mut items = ItemFlag{ flag:0 };
    
    items.add(&a);
    items.add(&b);
    items.add(&c);
    items.add(&z);
    
    items.add(&cap_a);
    items.add(&cap_x);
    items.add(&cap_y);
    items.add(&cap_z);
    
    // println!("ItemFlags:  {}", items);
    assert_eq!(items.flag, 0b11100000000000000000000001_10000000000000000000000111_u64);
}

#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));

    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "157");
    let output = solve(Box::new(lines.clone()), Part::Part2);
    assert_eq!(output.as_str(), "70");
}
