use helper::InputReader;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let mut container = Container::default();
        container.read("./test.txt").unwrap();

        assert_eq!(container.calculate_1(), 5934);

        assert_eq!(container.calculate_2(), 26984457539);
    }
}

fn main() {
    let mut container = Container::default();
    container.read("./input.txt").unwrap();

    let res = container.calculate_1();

    println!("result 1 : {} ", res);

    let res = container.calculate_2();

    println!("result 2 : {} ", res);
}

#[derive(Debug, Default)]
struct Container {
    generation: Generation,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        for fish in line.split(',') {
            match fish {
                "0" => self.generation.zero += 1,
                "1" => self.generation.one += 1,
                "2" => self.generation.two += 1,
                "3" => self.generation.tree += 1,
                "4" => self.generation.four += 1,
                "5" => self.generation.five += 1,
                "6" => self.generation.six += 1,
                "7" => self.generation.seven += 1,
                "8" => self.generation.eight += 1,
                &_ => {
                    panic!("not handle")
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Generation {
    zero: usize,
    one: usize,
    two: usize,
    tree: usize,
    four: usize,
    five: usize,
    six: usize,
    seven: usize,
    eight: usize,
}

impl Container {
    fn calculate_1(&self) -> usize {
        let mut generation = self.generation.clone();
        for _i in 0..80 {
            generation = generation.age();
        }

        generation.total()
    }
    fn calculate_2(&self) -> usize {
        let mut generation = self.generation.clone();
        for _i in 0..256 {
            generation = generation.age();
        }

        generation.total()
    }
}

impl Generation {
    fn age(&self) -> Self {
        Generation {
            zero: self.one,
            one: self.two,
            two: self.tree,
            tree: self.four,
            four: self.five,
            five: self.six,
            six: self.seven + self.zero,
            seven: self.eight,
            eight: self.zero,
        }
    }
    fn total(&self) -> usize {
        return self.zero
            + self.one
            + self.two
            + self.tree
            + self.four
            + self.five
            + self.six
            + self.seven
            + self.eight;
    }
}
