use std::io::{stdin, stdout, Result, Write};

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

fn main() {

    let mut lines = stdin().lines();
    
    let total = overlapped_count(&mut lines);

    let mut out = stdout().lock();
    out.write_all(format!("{}\n", total).as_bytes())
        .expect("should be able to write to stdout");
}

/// make sure we pass in `N+1` otherwise, we'll constantly overwrite the `N`th value instead of pushing it out.
fn overlapped_count(lines: &mut dyn Iterator<Item=Result<String>>) -> usize {
    
    let mut overlap_total: usize = 0;

    while let Some(Ok(line)) = lines.next() {
        if let Some((left, right)) = line.split_once(',') {
            let elf1:Range = left.into();
            let elf2:Range = right.into();

            // if elf1.fully_overlaps(&elf2) ||
            //     elf2.fully_overlaps(&elf1) {
            //     overlap_total += 1;
            // }
            if elf1.has_overlap(&elf2) {
                overlap_total += 1;
            }
        }
    }

    overlap_total
}