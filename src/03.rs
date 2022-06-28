fn step1(input: &Vec<&str>, digits: usize) {
    let (gamma, _) = input
        .iter()
        .map(|line| line.chars())
        .fold(vec![0; digits], |mut acc, number| {
            number
                .enumerate()
                .for_each(|(i, num)| match num {
                    '1' => acc[i] += 1,
                    _ => (),
                });
            acc
        })
        .iter()
        .map(|&num| ((num as f32 / input.len() as f32) + 0.5) as u32)
        .fold((0, 0), |(val, power), digit| (val + digit * 2_u32.pow(digits as u32 - power - 1), power + 1));

    let mask: u32 = (0..digits).fold(0, |acc, _| (acc << 1) | 1);

    let epsilon: u32 = !gamma & mask;
    let power_consumption: u32 = epsilon * gamma;

    println!("power_consumption: {}", power_consumption);
}

#[derive(Default)]
struct Node {
    zero: Option<Box<Node>>,
    one: Option<Box<Node>>,
}

impl Node {
    pub fn size(&self) -> usize {
        if self.zero.is_none() && self.one.is_none() {
            return 1;
        }

        let mut s = 0;
        if let Some(ref n) = self.zero {
            s += n.size();
        }
        if let Some(ref n) = self.one {
            s += n.size();
        }
        s
    }

    pub fn oxygen_generator_rating(&self) -> Vec<u32> {
        if self.zero.is_none() && self.one.is_none() {
            return Vec::new();
        }

        if self.zero.is_none() {
            let mut v = self.one.as_ref().unwrap().oxygen_generator_rating();
            v.push(1);
            return v;
        } else if self.one.is_none() {
            let mut v = self.zero.as_ref().unwrap().oxygen_generator_rating();
            v.push(1);
            return v;
        }

        let zero_size = self.zero.as_ref().unwrap().size();
        let one_size = self.one.as_ref().unwrap().size();

        if zero_size > one_size {
            let mut v = self.zero.as_ref().unwrap().oxygen_generator_rating();
            v.push(0);
            return v;
        } else {
            let mut v = self.one.as_ref().unwrap().oxygen_generator_rating();
            v.push(1);
            return v;
        }
    }

    pub fn co2_scrubber_rating(&self) -> Vec<u32> {
        if self.zero.is_none() && self.one.is_none() {
            return Vec::new();
        }

        if self.zero.is_none() {
            let mut v = self.one.as_ref().unwrap().co2_scrubber_rating();
            v.push(1);
            return v;
        } else if self.one.is_none() {
            let mut v = self.zero.as_ref().unwrap().co2_scrubber_rating();
            v.push(1);
            return v;
        }

        let zero_size = self.zero.as_ref().unwrap().size();
        let one_size = self.one.as_ref().unwrap().size();

        if zero_size <= one_size {
            let mut v = self.zero.as_ref().unwrap().co2_scrubber_rating();
            v.push(0);
            return v;
        } else {
            let mut v = self.one.as_ref().unwrap().co2_scrubber_rating();
            v.push(1);
            return v;
        }
    }
}


fn step2(input: &Vec<&str>, digits: usize) {
    let mut root = Box::new(Node::default());

    input
        .iter()
        .map(|line| line.chars())
        .map(|chars| chars.map(|c| c.to_digit(10).unwrap()).collect())
        .for_each(|digits: Vec<u32>| {
            digits
                .iter()
                .fold(&mut root, |node: &mut Box<Node>, val| match val {
                    0 => if let Some(ref mut branch) = node.zero {
                        return branch
                    } else {
                        let b = Box::new(Node::default());
                        node.zero = Some(b);
                        return node.zero.as_mut().unwrap()
                    },
                    1 => if let Some(ref mut branch) = node.one {
                        return branch
                    } else {
                        let b = Box::new(Node::default());
                        node.one = Some(b);
                        return node.one.as_mut().unwrap()
                    },
                    _ => panic!("This should have never happened!"),
                });
        });

    let (oxygen_generator_rating, _) = root
        .oxygen_generator_rating()
        .iter()
        .rev()
        .fold((0, 0), |(val, power), digit| (val + digit * 2_u32.pow(digits as u32 - power - 1), power + 1));

    let (co2_scrubber_rating, _) = root
        .co2_scrubber_rating()
        .iter()
        .rev()
        .fold((0, 0), |(val, power), digit| (val + digit * 2_u32.pow(digits as u32 - power - 1), power + 1));

    let life_support_rating: u32 = oxygen_generator_rating * co2_scrubber_rating;

    println!("life_support_rating is {}", life_support_rating);
}

fn main() {
    let input: Vec<&str> = include_str!("../input/03.txt")
    //let input: Vec<&str> = include_str!("../input/03-sample.txt")
        .trim()
        .lines()
        .collect();

    let digits = input.iter().next().unwrap().len();

    step1(&input, digits);
    step2(&input, digits);
}
