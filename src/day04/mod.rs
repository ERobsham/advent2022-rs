use crate::Part;

struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn fully_overlaps(&self, other: &Range) -> bool {
        self.start <= other.start &&
        self.end >= other.end
    }
    
    fn has_overlap(&self, other: &Range) -> bool {
        !(self.start > other.end ||
          other.start > self.end)  
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        if let Some((left, right)) = value.split_once('-') {
            let start:usize = left.parse().unwrap_or(0);
            let end:usize = right.parse().unwrap_or(0);

            Self{ start: start.min(end), end: end.max(start) }
        } else {
            Self{ start: 0, end: 0 }
        }
    }
}


pub(crate) fn solve(mut input: Box<dyn Iterator<Item = String>>, part: Part) -> String {
    
    let mut overlap_total: usize = 0;

    while let Some(line) = input.next() {
        if let Some((left, right)) = line.split_once(',') {
            let elf1:Range = left.into();
            let elf2:Range = right.into();

            if match part {
              Part::Part1 => elf1.fully_overlaps(&elf2) || elf2.fully_overlaps(&elf1),
              Part::Part2 => elf1.has_overlap(&elf2),
            } {
                overlap_total += 1;
            }
        }
    }

    format!("{}", overlap_total)
}


#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));

    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "2");
    
    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "4");
}
