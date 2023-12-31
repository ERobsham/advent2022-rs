
use std::{collections::VecDeque, ops::{AddAssign, SubAssign}};

use crate::Part;

const START_CYCLE_PT1:usize = 20;
const CYCLE_INTERVAL:usize = 40;

const ADDX_CYCLES:usize = 2;
const NOOP_CYCLES:usize = 1;

const LINE_WIDTH:usize = 40;

type CyclesRemaining = usize;
type CycleToFire = usize;
enum Instruction {
    AddX(isize),
    Noop,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if let Some((_left, right)) = value.split_once(' ') {
            let x:isize = right.parse().unwrap_or(0);
            Instruction::AddX(x)
        } else {
            Instruction::Noop
        }
    }
}


struct Interrupt {
    interval: usize,
    repeats: bool,
}

enum CycleState {
    Processing,
    Interrupted,
    WorkQueueEmpty,
}

struct CPU {
    cycle: usize,
    register_x: isize,
    line_buffer: Vec<char>,
    
    current_instruction: Option<(Instruction, CyclesRemaining)>,
    instruction_queue: VecDeque<Instruction>,

    interrupt: Option<(Interrupt, CycleToFire)>,
}

impl CPU {

    fn new() -> Self {
        Self { 
            cycle: 1,
            register_x: 1, 
            line_buffer: Vec::new(),
            current_instruction: None, 
            instruction_queue: VecDeque::new(), 
            interrupt: None,
        }
    }

    fn install_interrupt(&mut self, interrupt: Interrupt) {
        let cycle_to_fire = self.cycle + interrupt.interval;
        self.interrupt = Some((interrupt, cycle_to_fire));
    }

    fn schedule_instruction(&mut self, i: Instruction) {
        self.instruction_queue.push_back(i);
    }

    // Returns true if successful and false if 
    // there were no more instructions left to dequeue.
    fn dequeue_instruction(&mut self) {
        if self.current_instruction.is_some() { 
            // don't overwrite a currently processing instruction.
            return; 
        }

        if let Some(i) = self.instruction_queue.pop_front() {
            let next_instruction = match i {
                Instruction::AddX(_x) => (i, ADDX_CYCLES),
                Instruction::Noop => (i, NOOP_CYCLES),
            };
            
            self.current_instruction = Some(next_instruction);
        }
    }

    fn perform_cycle(&mut self) -> CycleState {

        // first, ensure we have an instruction ready to process.
        self.dequeue_instruction();
        
        // next, handle interrupts before the cycle 'runs'.
        if let Some((interrupt, cycle_to_fire)) = &mut self.interrupt {
            if &self.cycle == cycle_to_fire {

                if interrupt.repeats {
                    cycle_to_fire.add_assign(interrupt.interval);
                } else {
                    self.interrupt = None;
                }

                return CycleState::Interrupted;
            }
        }

        // write the line buffer before the Instruction is 'run'.
        let x1 = self.register_x-1;
        let x2 = self.register_x+1;
        let lx = ((self.cycle-1) % LINE_WIDTH) as isize;
        let c = if lx >= x1 && lx <= x2 { '#' } else { '.' };
        self.line_buffer.push(c);

        // finally, perform the work
        match &mut self.current_instruction {
            Some((instruction, cycles_remaining)) => {
                if *cycles_remaining > 1 {
                    cycles_remaining.sub_assign(1);
                } else {
                    if let Instruction::AddX(x) = instruction {
                        self.register_x += *x;
                    }

                    self.current_instruction = None;
                }

                // increment the cycle counter anytime we process anything.
                self.cycle += 1;
                CycleState::Processing
            },
            None => {
                CycleState::WorkQueueEmpty
            },
        }
    }

    fn run_until_interrupt(&mut self) -> CycleState {
        loop {
            let state = self.perform_cycle();
            match state {
                CycleState::Processing => {},
                _ => return state,
            }
        }
    }

    fn current_signal_strength(&self) -> isize {
        (self.cycle as isize) * self.register_x
    }

    // gets the current state of the line buffer (with '\n' appended to it),
    // and then clears the line buffer.
    fn get_line_buffer(&mut self) -> String {
        let line = 
            self.line_buffer
                .iter()
                .fold(String::new(), |mut acc, c| { 
                    acc.push(*c);
                    acc
                });
        self.line_buffer.clear();
        line + "\n"
    }

}


pub(crate) fn solve(mut input: Box<dyn Iterator<Item = String>>, part: Part) -> String {

    let mut cpu = CPU::new();

    while let Some(line) = input.next() {
        let instruction = Instruction::from(line.as_str());
        cpu.schedule_instruction(instruction);
    }
    
    match part {
        Part::Part1 => calc_signal_strength_totals(cpu, START_CYCLE_PT1, CYCLE_INTERVAL),
        Part::Part2 => accumulate_line_buffers(cpu, CYCLE_INTERVAL, CYCLE_INTERVAL),
    }
}

fn calc_signal_strength_totals(mut cpu: CPU, start: usize, interval: usize) -> String {
    
    cpu.install_interrupt(Interrupt { interval: start-1, repeats: false });
    cpu.run_until_interrupt();

    cpu.install_interrupt(Interrupt { interval, repeats: true });

    let mut total = cpu.current_signal_strength();
    loop {
        match cpu.run_until_interrupt() {
            CycleState::Interrupted => total += cpu.current_signal_strength(),
            _ => break,
        }
    }

    format!("{}", total)
}

fn accumulate_line_buffers(mut cpu: CPU, _start: usize, interval: usize) -> String {
    
    let mut lines = String::new();
    cpu.install_interrupt(Interrupt { interval, repeats: true });
    
    loop {
        match cpu.run_until_interrupt() {
            CycleState::Interrupted => lines += &cpu.get_line_buffer(),
            _ => break,
        }
    }

    lines.trim_end().to_string()
}


#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));

    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "13140");
    
    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), 
r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
}

#[cfg(test_output)]
#[test]
fn test_input_file() {
    const EXAMPLE: &str = r"input/day-10";

    let input_file = std::fs::read_to_string(EXAMPLE).unwrap();
    let lines:Vec<String> = input_file.split('\n')
        .map(|item| String::from(item)).collect();
    let input = Box::new(lines.into_iter());
        
    
    let output = solve(input, Part::Part2);
    println!("{}", output);
}
