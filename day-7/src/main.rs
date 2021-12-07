use helper::InputReader;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let mut container = Container::default();
        container.read("./test.txt").unwrap();

        assert_eq!(container.calculate_1(), 37);

        assert_eq!(container.calculate_2(), 168);
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
    positions: Vec<isize>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        self.positions = line
            .split(',')
            .into_iter()
            .map(|v| v.parse::<isize>().unwrap())
            .collect();
    }
}

impl Container {
    fn calculate_1(&self) -> isize {
        let mut pos = self.positions.clone();

        pos.sort();
        let median = pos[pos.len() / 2];

        pos.into_iter().map(|v| (v - median).abs()).sum()
    }

    fn calculate_2(&self) -> isize {
        let mut pos = self.positions.clone();

        pos.sort();
        let moy: f32 =
            (pos.clone().into_iter().map(|v| v as f32).sum::<f32>() / pos.len() as f32).round();
        let moy = moy as isize;

        let mut possible_res: Vec<isize> = Vec::new();
        for i in moy - 3..moy + 3 {
            possible_res.push(Self::get_fuel(pos.clone(), i));
        }

        possible_res.into_iter().reduce(|a, b| a.min(b)).unwrap()
    }

    fn get_fuel(pos: Vec<isize>, moy: isize) -> isize {
        pos.into_iter()
            .map(|v| {
                let step = (v - moy).abs();
                let mut res = 0;
                for i in 0..step + 1 {
                    res += i;
                }
                res
            })
            .sum()
    }
}
