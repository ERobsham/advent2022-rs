
use std::{collections::HashSet, rc::Rc, cell::RefCell};

use crate::Part;


trait Move {
    fn apply_movement(&mut self, direction: &Movement);
}

enum Movement {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl From<&str> for Movement {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(' ')
            .expect("input should be in the form of '<direction> <moves>'");

        let dist: i32 = right.parse()
            .expect("<moves> should be a number");

        match left {
            "U" => Movement::Up(dist),
            "D" => Movement::Down(dist),
            "L" => Movement::Left(dist),
            "R" => Movement::Right(dist),
            _ => Movement::Up(0),
        }
    }
}


#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct Loc {
    x: i32,
    y: i32,
}

impl Move for Loc {
    fn apply_movement(&mut self, direction: &Movement) {
        let (dx, dy):(i32,i32) = match direction {
            Movement::Up(dist) => (0, dist.clone()),
            Movement::Down(dist) => (0, dist * -1),
            Movement::Left(dist) => (dist * -1, 0),
            Movement::Right(dist) => (dist.clone(), 0),
        };

        self.x += dx;
        self.y += dy;
    }
}

impl Loc {
    fn step_towards(&mut self, other: &Loc) {
        let (mut dx, mut dy) = (other.x - self.x, other.y - self.y);
        
        if dx > 0 { dx = 1 }
        if dx < 0 { dx = -1 }
        if dy > 0 { dy = 1 }
        if dy < 0 { dy = -1 }

        self.x += dx;
        self.y += dy;
    }

    fn square_dist(&self, other: &Loc) -> i32 {
        let (dx, dy) = (other.x.abs_diff(self.x) as i32, other.y.abs_diff(self.y) as i32);

        // this assumes 1 diagonal unit == 1 unit
        // (ie: [2,2] only takes 2 moves to traverse)
        (dx + dy) - (dx.min(dy))
    }
}


struct Snake {
    body: Rc<RefCell<SnakeSegment>>,
    tail_trail: HashSet<Loc>,
}

impl Snake {
    fn new(len: usize) -> Self {
        let mut snake = Snake { 
            body: Rc::new(RefCell::new( SnakeSegment::new(len) )),
            tail_trail: HashSet::default(),
        };

        // make sure we always capture the start location
        snake.tail_trail.insert(Loc::default());

        snake
    }
}

impl Move for Snake {
    fn apply_movement(&mut self, direction: &Movement) {
        let mut dest = self.body.borrow().head_loc();
        dest.apply_movement(direction);
        let dest = dest; // just to drop the mut

        while self.body.borrow().head_loc().square_dist(&dest) > 0 {

            {   
                // using these braces to keep the mutable borrow 
                // narrowly scoped.  Otherwise this would panic 
                // once we hit the first `curr_seg.borrow()`

                let head = &mut self.body.borrow_mut().loc;
                head.step_towards(&dest);
            }
            
            let mut curr_seg = self.body.clone();
            let mut next_seg = curr_seg.borrow().next.clone();
            loop {
                
                match next_seg {
                    Some(seg) => {
                        if seg.borrow().loc.square_dist(&curr_seg.borrow().loc) > 1 {
                            seg.borrow_mut().loc.step_towards(&curr_seg.borrow().loc);
                            // self.tail_trail.insert(self.tail.clone());
                        }
        
                        next_seg = seg.borrow().next.clone();
                        curr_seg = seg.clone();
                    },
                    None => break,
                }

            }

            self.tail_trail.insert(curr_seg.borrow().tail_loc());
        }
    }
}

struct SnakeSegment {
    loc: Loc,
    next: Option<Rc<RefCell<SnakeSegment>>>,
}

impl SnakeSegment {

    fn new(len: usize) -> Self {
        match len {
            2..=usize::MAX => Self { loc: Loc::default(), next: Some(Rc::new(RefCell::new(Self::new(len-1)))) },
            1 => Self { loc: Loc::default(), next: None },
            _ => unreachable!(),
        }
    }

    fn head_loc(&self) -> Loc {
        self.loc.clone()
    }

    fn tail_loc(&self) -> Loc {
        if let Some(next) = self.next.clone() {
            next.borrow().tail_loc()
        } else {
            self.loc.clone()
        }
    }

}

pub(crate) fn solve(mut input: Box<dyn Iterator<Item = String>>, part: Part) -> String {
    
    let mut snake = match part {
        Part::Part1 => Snake::new(2),
        Part::Part2 => Snake::new(10),
    };
    
    while let Some(line) = input.next() {
        let movement: Movement = line.as_str().into();

        snake.apply_movement(&movement);
    }

    format!("{}", snake.tail_trail.len())
}




#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));

    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "13");
}

#[test]
// sanity check vs example input for pt2
fn test_input_pt2() {
    const EXAMPLE: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));

    
    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "36");
}

#[test]
fn test_sq_dist() {
    let loc1 = Loc::default();
    assert!(loc1.square_dist(&loc1) == 0);

    let loc2 = Loc{x: 0, y: 10};
    assert!(loc1.square_dist(&loc2) == 10);
    assert!(loc1.square_dist(&loc2) == loc2.square_dist(&loc1));

    let loc2 = Loc{x: 1, y: 10};
    assert!(loc1.square_dist(&loc2) == 10);
    assert!(loc1.square_dist(&loc2) == loc2.square_dist(&loc1));

    let loc2 = Loc{x: 10, y: 10};
    assert!(loc1.square_dist(&loc2) == 10);
    assert!(loc1.square_dist(&loc2) == loc2.square_dist(&loc1));
}