use std::{char::ParseCharError, collections::VecDeque, str::FromStr, fmt::format};

#[derive(Debug)]
struct Transaction {
    item: usize,
    monkey: usize,
}

#[derive(Debug)]
enum Operator {
    Add,
    Mpy,
}

#[derive(Debug)]
enum Operand {
    Literal(usize),
    Old,
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

impl Operation {
    pub fn eval(&self, old: usize) -> usize {
        let operand = match self.operand {
            Operand::Literal(operand) => operand,
            Operand::Old => old,
        };

        match self.operator {
            Operator::Add => old + operand,
            Operator::Mpy => old * operand,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: usize,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize,
}

impl FromStr for Monkey {
    type Err = ParseCharError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        lines.next().unwrap(); // Monkey #:
        let mut items: VecDeque<usize> = VecDeque::new();
        lines
            .next()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .for_each(|item| items.push_back(item.trim().parse::<usize>().unwrap()));

        let mut op_iter = lines.next().unwrap().trim().split(' ').skip(4);
        let operator = op_iter.next().unwrap();
        let operand = op_iter.next().unwrap();
        let operand = match operand {
            "old" => Operand::Old,
            operand => Operand::Literal(operand.parse::<usize>().unwrap()),
        };
        let operator: Operator = match operator {
            "+" => Operator::Add,
            "*" => Operator::Mpy,
            operator => panic!("unexpected operator: {}", operator),
        };
        let operation = Operation { operator, operand };

        let test = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .nth(3)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let true_monkey = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .nth(5)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let false_monkey = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .nth(5)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Ok(Self {
            items,
            operation,
            test,
            true_monkey,
            false_monkey,
            inspections: 0,
        })
    }
}

impl Monkey {
    fn operate_one(&mut self, item: usize, lcm: Option<usize>) -> Transaction {
        let item = self.operation.eval(item);
        let item = match lcm {
            None => item / 3,
            Some(lcm) => item % lcm,
        };
        let monkey = if item % self.test == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        };

        self.inspections += 1;
        Transaction { item, monkey }
    }

    pub fn operate(&mut self, lcm: Option<usize>) -> Vec<Transaction> {
        let mut txs: Vec<Transaction> = Vec::with_capacity(self.items.len());
        while !self.items.is_empty() {
            let item = self.items.pop_front().unwrap();
            let tx = self.operate_one(item, lcm);
            txs.push(tx);
        }
        txs
    }

    pub fn receive_item(&mut self, item: usize) {
        self.items.push_back(item)
    }

    pub fn inspections(&self) -> usize {
        self.inspections
    }

    pub fn test(&self) -> usize {
        self.test
    }
}

pub fn run(input: &str, part1: bool) -> String {
    let mut monkeys = input
        .split("\n\n")
        .map(|s| Monkey::from_str(s).unwrap())
        .collect::<Vec<_>>();

    let (rounds, lcm) = if part1 {
        (20, None)
    } else {
        let lcm = monkeys.iter().fold(1_usize, |lcm, monkey| lcm * monkey.test());
        (10000, Some(lcm))
    };

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let txs = monkey.operate(lcm);
            txs.into_iter().for_each(|tx| {
                let monkey = &mut monkeys[tx.monkey];
                monkey.receive_item(tx.item);
            });
        }
    }

    let mut monkey_inspections = monkeys
        .into_iter()
        .map(|m| m.inspections())
        .collect::<Vec<_>>();
    monkey_inspections.sort();
    let mut sorted_monkey = monkey_inspections.into_iter().rev();

    let result = sorted_monkey.next().unwrap() * sorted_monkey.next().unwrap();
    format!("{}", result)
}

pub fn part1(input: &str) -> String {
    run(input, true)
}

pub fn part2(input: &str) -> String {
    run(input, false)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Monkey 0:
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

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        assert_eq!(result, "10605");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, "2713310158");
    }
}
