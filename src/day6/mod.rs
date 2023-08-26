use std::fmt::Display;
use std::collections::VecDeque;

use crate::Part;

const LOWERCASE_OFFSET: u8 = 96;
const UPPERCASE_OFFSET: u8 = 38;
const DUP_MASK:u64 = 1 << 63; 

struct CharFlags {
    pub flag: u64,
}

impl CharFlags {

    // add a byte that represents the item as a character (a-z,A-Z)
    fn add(&mut self, item: &u8) -> Self {
        let offset:u8;
        match item {
            97..=122 => offset = item - LOWERCASE_OFFSET,
            65..=90 => offset = item - UPPERCASE_OFFSET,
            _ => { return CharFlags{ flag: self.flag.to_owned() }; },
        }

        let other_flag:u64 = 1 << (offset - 1);
        let is_dupe:bool = (self.flag & other_flag) != 0;

        self.flag |= 1 << (offset - 1);

        if is_dupe {
            self.flag |= DUP_MASK;
        }

        CharFlags { flag: self.flag.to_owned() }
    }

    fn has_dupes(&self) -> bool {
        self.flag & DUP_MASK != 0
    }

}

impl From<&VecDeque<u8>> for CharFlags {
    fn from(values: &VecDeque<u8>) -> Self {
        let mut flag = Self::default();
        values.iter().for_each(|v| { flag.add(v); } );
        flag
    }
}

impl Default for CharFlags {
    fn default() -> Self {
        CharFlags { flag: 0 }
    }
}

impl Display for CharFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MASK_26: u64 = 0b111111_1111111111_1111111111;

        let lower = self.flag & MASK_26;
        let upper = (self.flag >> 26) & MASK_26;
        f.write_fmt(format_args!("(has_dupes: {}) ({:026b} {:026b})", self.has_dupes(), upper, lower))
    }
}


pub(crate) fn solve(mut input: Box<dyn Iterator<Item = String>>, part: Part) -> String {
    
    let mut idxes:Vec<usize> = Vec::new();

    let packet_len: usize = match part {
        Part::Part1 => 4,
        Part::Part2 => 14,
    };

    while let Some(line) = input.next() {
        if let Some(idx) = find_start(&line, packet_len) {
            idxes.push(idx);
        }
    }

    format!("{:?}", idxes)
        .replace(",", "")
        .trim_matches(|c| c == '[' || c == ']')
        .to_string()
}

fn find_start(line: &String, len: usize) -> Option<usize> {

    let mut chars = line.char_indices();
    let mut buf: VecDeque<u8> = VecDeque::new();

    while let Some((i, c)) = chars.next() {
        let mut byte: [u8; 1] = [0];
        // SAFETY: could panic on unexpected values
        //         but our input is constrained to lowercase ascii characters
        c.encode_utf8(&mut byte);
        buf.push_back(byte.first().unwrap().to_owned());

        if buf.len() == len {
            let flags: CharFlags = (&buf).into();
            if !flags.has_dupes() {
                return Some(i + 1);
            }

            let _ = buf.pop_front(); // discard front
        }
    }

    None
}



#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));

    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "7 5 6 10 11");
    
    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "19 23 23 29 26");
}