use helper::InputReader;
use regex::Regex;

fn main() {
    let mut container = Container::default();
    container.read("./input.txt").unwrap();

    let position = container.calculate_1();

    println!("result 1 : {} --- {:?}", position.horizontal * position.depth, position );


    let position = container.calculate_2();

    println!("result 2 : {} --- {:?}", position.horizontal * position.depth, position );
}

#[derive(PartialEq, Debug)]
enum Move {
    Forward(usize),
    Down(usize),
    Up(usize),
}

struct Container {
    regex: Regex,
    movement: Vec<Move>,
}

#[derive(PartialEq, Debug)]
struct Position {
    horizontal: usize,
    depth: usize,
    aim: usize
}

impl Default for Container {
    fn default() -> Self {
        Container {
            regex: Regex::new(r"^(forward|down|up) (\d+)$").unwrap(),
            movement: Vec::new(),
        }
    }
}


impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        for cap in self.regex.captures_iter(line) {
            let dir = cap.get(1).expect("no direction");
            let dist = cap.get(2).expect("no distance");

            match dir.as_str() {
                "forward" => {
                    self.movement.push(Move::Forward(dist.as_str().parse::<usize>().unwrap()))
                }
                "down" => {
                    self.movement.push(Move::Down(dist.as_str().parse::<usize>().unwrap()))
                }
                "up" => {
                    self.movement.push(Move::Up(dist.as_str().parse::<usize>().unwrap()))
                }
                &_ => {
                    panic!("direction not recognize")
                }
            }
        }
    }
}

impl Container {
    fn calculate_1(&self) -> Position {
        let mut pos = Position {
            horizontal: 0,
            depth: 0,
            aim: 0
        };

        for m in self.movement.iter() {

            match m {
                Move::Forward(d) => {
                    pos.horizontal += d;
                }
                Move::Down(d) => {
                    pos.depth += d;
                }
                Move::Up(d) => {
                    pos.depth -= d;
                }
            }
        }

        pos
    }

    fn calculate_2(&self) -> Position {
        let mut pos = Position {
            horizontal: 0,
            depth: 0,
            aim: 0
        };

        for m in self.movement.iter() {

            match m {
                Move::Forward(d) => {
                    pos.horizontal += d;
                    pos.depth += pos.aim * d;
                }
                Move::Down(d) => {
                    pos.aim += d;
                }
                Move::Up(d) => {
                    pos.aim -= d;
                }
            }
        }

        pos
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let mut container = Container::default();
        container.read("./test.txt").unwrap();

        assert_eq!(container.calculate_1(), Position { horizontal: 15, depth: 10, aim:0 });

        assert_eq!(container.calculate_2(), Position { horizontal: 15, depth: 60, aim:10 });
    }
}
