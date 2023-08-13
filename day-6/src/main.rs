use std::collections::VecDeque;
use std::io::{stdin, stdout, Result, Write};
use std::fmt::Display;

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


fn main() {

    let mut lines = stdin().lines();
    
    let result = parse_packet_starting_idxes(&mut lines);

    let mut out = stdout().lock();
    out.write_all(format!("{}\n", result).as_bytes())
        .expect("should be able to write to stdout");
}

fn parse_packet_starting_idxes(lines: &mut dyn Iterator<Item=Result<String>>) -> String {

    let mut idxes:Vec<usize> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if let Some(idx) = find_start(&line) {
            idxes.push(idx);
        }
    }

    format_args!("{:?}", idxes).to_string()
}

fn find_start(line: &String) -> Option<usize> {

    let mut chars = line.char_indices();
    let mut buf: VecDeque<u8> = VecDeque::new();

    while let Some((i, c)) = chars.next() {
        let mut byte: [u8; 1] = [0];
        // SAFETY: could panic on unexpected values
        //         but our input is constrained to lowercase ascii characters
        c.encode_utf8(&mut byte);
        buf.push_back(byte.first().unwrap().to_owned());

        // pt1
        // if buf.len() == 4 {
        // pt2
        if buf.len() == 14 {
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
    let file_contents: String = (r"bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
").into();
    let file_contents: Result<String> = Ok(file_contents);

    match file_contents {
        Ok(input) => {
            let mut lines = input.split('\n')
                .map(|item| Ok(String::from(item)));


            let output = parse_packet_starting_idxes(&mut lines);

            println!("output : {}", output);
        },
        Err(err) => println!("couldnt read input: {:?}", err),
    }

}