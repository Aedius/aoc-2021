use helper::InputReader;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let mut container = Container::default();
        container.read("./test.txt").unwrap();
        assert_eq!(container.calculate_1(), 17);

        let mut container = Container::default();
        container.read("./test.txt").unwrap();
        assert_eq!(container.calculate_2(), 0);
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
    folds: Vec<Fold>,
    points: Vec<(isize, isize)>,
    map: Vec<Vec<bool>>,
    size: (usize, usize),
}

#[derive(Debug)]
enum Fold {
    Y(isize),
    X(isize),
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.contains(",") {
            let r: Vec<isize> = line.split(",").into_iter().map(|v| v.parse::<isize>().unwrap()).collect();

            self.points.push((r[0], r[1]));
        } else if line.contains("fold along") {
            let r: Vec<&str> = line.split(" ").into_iter().collect();

            let axe: Vec<&str> = r[2].split("=").into_iter().collect();

            if axe[0] == "x" {
                self.folds.push(Fold::X(axe[1].parse::<isize>().unwrap()))
            } else {
                self.folds.push(Fold::Y(axe[1].parse::<isize>().unwrap()))
            }
        }
    }
}

impl Container {
    fn calculate_1(&mut self) -> isize {
        self.calculate_map();
        self.fold();

        let mut res = 0;
        for r in &self.map {
            for b in r {
                if *b {
                    res += 1;
                }
            }
        }
        res
    }
    fn calculate_2(&mut self) -> isize {
        self.calculate_map();

        while self.fold().is_some() {}


        self.display_map();
        0
    }

    fn fold(&mut self) -> Option<()> {
        if self.folds.is_empty() {
            return None;
        }

        let fold = self.folds.remove(0);

        match fold {
            Fold::Y(n) => {
                let mut map = vec![vec![false; self.size.0]; n as usize];
                for i in 0..n {
                    for x in 0..self.size.0 {
                        let i = i as usize;
                        map[i][x] = self.map[i][x] || self.map[self.size.1 - i - 1 as usize][x];
                    }
                }
                self.size = (self.size.0, n as usize);
                self.map = map;
            }
            Fold::X(n) => {
                let mut map = vec![vec![false; n as usize]; self.size.1];
                for y in 0..self.size.1 {
                    for i in 0..n {
                        let i = i as usize;
                        map[y][i] = self.map[y][i] || self.map[y][self.size.0 - i - 1 as usize];
                    }
                }
                self.size = (n as usize, self.size.1);
                self.map = map;
            }
        }


        Some(())
    }

    fn calculate_map(&mut self) {
        let max = self.points.clone().into_iter().reduce(|a, b| (a.0.max(b.0), a.1.max(b.1))).unwrap();

        self.size = ((max.0 + 1) as usize, (max.1 + 1) as usize);

        self.map = vec![vec![false; self.size.0]; self.size.1];

        for (x, y) in &self.points {
            self.map[*y as usize][*x as usize] = true;
        }
    }

    fn display_map(&self) {
        for row in &self.map {
            for b in row {
                match b {
                    true => {
                        print!("##")
                    }
                    false => {
                        print!("  ")
                    }
                }
            }
            println!();
        }
        println!();
    }
}