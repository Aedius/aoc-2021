use helper::InputReader;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let mut container = Container::default();
        container.read("./test.txt").unwrap();

        assert_eq!(container.calculate_1(), 26);

        assert_eq!(Data::contain("bcdefg".to_string(), "ab".to_string()), false);

        let data = Data {
            patterns: ["acedgfb".to_string(), "cdfbe".to_string(), "gcdfa".to_string(), "fbcad".to_string(), "dab".to_string(), "cefabd".to_string(), "cdfgeb".to_string(), "eafb".to_string(), "cagedb".to_string(), "ab".to_string()],
            outputs: ["cdfeb".to_string(), "fcadb".to_string(), "cdfeb".to_string(), "cdbaf".to_string()],
        };

        assert_eq!(data.solve(), 5353);

        assert_eq!(container.calculate_2(), 61229);
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
struct Data {
    patterns: [String; 10],
    outputs: [String; 4],
}

#[derive(Debug, Default)]
struct Container {
    data: Vec<Data>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        let mut split = line.split("|");
        let part1: Vec<String> = split.next().unwrap().split(" ").into_iter().filter(|v| !v.is_empty()).map(|v| v.to_string()).collect();
        let part2: Vec<String> = split.next().unwrap().split(" ").into_iter().filter(|v| !v.is_empty()).map(|v| v.to_string()).collect();

        let data = Data {
            patterns: part1.try_into().unwrap_or_else(|v: Vec<String>| panic!("Expected a Vec of length 10 but it was {}", v.len())),
            outputs: part2.try_into().unwrap_or_else(|v: Vec<String>| panic!("Expected a Vec of length 4 but it was {}", v.len())),
        };

        self.data.push(data);
    }
}

impl Container {
    fn calculate_1(&self) -> isize {
        let mut res = 0;

        for row in &self.data {
            let f: Vec<&String> = row.outputs.iter().filter(|v| [2, 3, 4, 7].contains(&v.len())).collect();

            res += f.len();
        }


        res.try_into().unwrap()
    }

    fn calculate_2(&self) -> isize {
        let mut res = 0;

        for d in &self.data {
            res += d.solve();
        }

        res
    }
}

impl Data {
    fn solve(&self) -> isize {
        let mut zero = "".to_string();
        let mut one = "".to_string();
        let mut two = "".to_string();
        let mut three = "".to_string();
        let mut four = "".to_string();
        let mut five = "".to_string();
        let mut six = "".to_string();
        let mut seven = "".to_string();
        let mut eight = "".to_string();
        let mut nine = "".to_string();

        let mut size5 = Vec::new(); // 2 3 5
        let mut size6 = Vec::new(); // 0 6 9

        for pattern in &self.patterns {
            match pattern.len() {
                2 => {
                    one = Self::sort(pattern);
                }
                3 => {
                    seven = Self::sort(pattern);
                }
                4 => {
                    four = Self::sort(pattern);
                }
                5 => {
                    size5.push(Self::sort(pattern));
                }
                6 => {
                    size6.push(Self::sort(pattern));
                }
                7 => {
                    eight = Self::sort(pattern);
                }
                _ => {
                    panic!("size not covered")
                }
            }
        }

        let mut i = 0;
        for (k, s5) in size5.iter().enumerate() {
            if Self::contain(s5.clone(), one.clone()) {
                three = s5.clone();
                i = k;
            }
        }
        size5.remove(i);

        for s6 in &size6 {
            if !Self::contain(s6.clone(), one.clone()) {
                six = s6.clone()
            } else if Self::contain(s6.clone(), three.clone()) {
                nine = s6.clone()
            } else {
                zero = s6.clone()
            }
        }

        for s5 in &size5 {
            if Self::contain(nine.clone(), s5.clone()) {
                five = s5.clone();
            } else {
                two = s5.clone()
            }
        }

        let mut res = "".to_string();


        for out in &self.outputs {
            if Self::sort(out) == zero {
                res = format!("{}0", res);
            }
            if Self::sort(out) == one {
                res = format!("{}1", res);
            }
            if Self::sort(out) == two {
                res = format!("{}2", res);
            }
            if Self::sort(out) == three {
                res = format!("{}3", res);
            }
            if Self::sort(out) == four {
                res = format!("{}4", res);
            }
            if Self::sort(out) == five {
                res = format!("{}5", res);
            }
            if Self::sort(out) == six {
                res = format!("{}6", res);
            }
            if Self::sort(out) == seven {
                res = format!("{}7", res);
            }
            if Self::sort(out) == eight {
                res = format!("{}8", res);
            }
            if Self::sort(out) == nine {
                res = format!("{}9", res);
            }
        }

        res.parse::<isize>().unwrap()
    }

    fn sort(pattern: &String) -> String {
        pattern.chars().sorted().collect::<String>()
    }

    fn contain(long: String, short: String) -> bool {
        let mut contain = true;

        for v in short.chars() {
            if !long.contains(v) {
                contain = false
            }
        }


        contain
    }
}
