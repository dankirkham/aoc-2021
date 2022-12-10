use std::fmt::Write;

#[derive(Debug)]
struct State {
    pub x: i32,
    pub row: usize,
    pub col: usize,
    pub cycle: i32,
    pub pixels: Vec<bool>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            x: 1,
            row: 0,
            col: 0,
            cycle: 1,
            pixels: vec![false; 40 * 6],
        }
    }
}

impl State {
    pub fn signal_strength(&self) -> i32 {
        self.x * self.cycle
    }

    fn mov(&mut self) {
        self.col += 1;
        if self.col > 39 {
            self.col = 0;
            self.row += 1;
            if self.row > 5 {
                self.row = 0;
            }
        }
    }

    fn draw(&mut self) {
        if self.x >= (self.col as i32) - 1 && self.x <= (self.col as i32) + 1 {
             self.pixels[self.row * 40 + self.col] = true;
        }
    }

    pub fn tick(&mut self) {
        self.draw();
        self.mov();
    }

    pub fn stringify(&self) -> String {
        let mut s = String::new();
        let mut col = 0;
        for pixel in &self.pixels {
            if *pixel {
                write!(&mut s, "#");
            } else {
                write!(&mut s, ".");
            }
            col += 1;
            if col >= 40 {
                writeln!(&mut s);
                col = 0;
            }
        }
        s
    }
}

trait Task {
    /// Returns true when done
    fn tick(&mut self, state: &mut State) -> bool;
}

pub enum Operation {
    Noop,
    Addx(i32),
}

struct AddxTask {
    ticks: usize,
    arg: i32,
}

impl AddxTask {
    pub fn new(arg: i32) -> Self {
        Self {
            ticks: 1,
            arg
        }
    }
}

impl Task for AddxTask {
    fn tick(&mut self, state: &mut State) -> bool {
        state.cycle += 1;
        if self.ticks > 0 {
            self.ticks -= 1;
            false
        } else {
            self.ticks = 0;
            state.x += self.arg;
            true
        }
    }
}

struct NoopTask;

impl Task for NoopTask {
    fn tick(&mut self, state: &mut State) -> bool {
        state.cycle += 1;
        true
    }
}

impl Into<Box<dyn Task>> for Operation {
    fn into(self) -> Box<dyn Task> {
        match self {
            Self::Noop => Box::new(NoopTask),
            Self::Addx(arg) => Box::new(AddxTask::new(arg)),
        }
    }
}

pub fn read_program(input: &str) -> impl Iterator<Item = Operation> + '_ {
    input.lines().map(|line| {
        let mut tokens = line.split(' ');
        let op = tokens.next().unwrap();
        match op {
            "addx" => {
                let arg = tokens.next().unwrap().parse::<i32>().unwrap();
                Operation::Addx(arg)
            },
            "noop" => Operation::Noop,
            _ => panic!()
        }
    })
}

pub fn part1(input: &str) -> String {
    let mut ops = read_program(input);
    let mut state = State::default();
    let mut task: Box<dyn Task> = ops.next().unwrap().into();
    let mut sum: i32 = 0;
    for _ in 0..19 {
        if task.tick(&mut state) {
            task = ops.next().unwrap().into();
        }
    }
    let signal_strength = state.signal_strength();
    sum += signal_strength;
    if task.tick(&mut state) {
        task = ops.next().unwrap().into();
    }

    for _ in 0..5 {
        for _ in 0..39 {
            if task.tick(&mut state) {
                task = ops.next().unwrap().into();
            }
        }
        let signal_strength = state.signal_strength();
        sum += signal_strength;
        if task.tick(&mut state) {
            task = ops.next().unwrap().into();
        }
    }

    format!("{}", sum)
}

pub fn part2(input: &str) -> String {
    let mut ops = read_program(input);
    let mut state = State::default();
    let mut task: Box<dyn Task> = ops.next().unwrap().into();

    loop {
        state.tick();
        let done_with_op = task.tick(&mut state);
        if done_with_op {
            if let Some(op) = ops.next() {
                task = op.into();
            } else {
                break;
            }
        }
    }

    let result = state.stringify();

    format!("{}", result)
}

#[cfg(test)]
mod test {
    use super::*;

    const SIMPLE: &str = "noop
addx 3
addx -5";

    const INPUT: &str = "addx 15
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

    #[test]
    fn test_simple() {
        let mut ops = read_program(SIMPLE);
        let mut state = State::default();
        let mut task: Box<dyn Task> = ops.next().unwrap().into();

        if task.tick(&mut state) {
            task = ops.next().unwrap().into();
        }
        assert_eq!(state.x, 1);

        if task.tick(&mut state) {
            task = ops.next().unwrap().into();
        }
        assert_eq!(state.x, 1);

        if task.tick(&mut state) {
            task = ops.next().unwrap().into();
        }
        assert_eq!(state.x, 4);

        if task.tick(&mut state) {
            task = ops.next().unwrap().into();
        }
        assert_eq!(state.x, 4);

        task.tick(&mut state);
        assert_eq!(state.x, -1);
    }

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        assert_eq!(result, "13140");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");
    }
}
