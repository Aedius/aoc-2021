use helper::InputReader;
use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let mut container = Container::default();
        container.read("./test.txt").unwrap();

        assert_eq!(
            container.calculate_1(),
            5
        );

        assert_eq!(
            container.calculate_2(),
            12
        );
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

#[derive(Debug)]
struct Container {
    regex: Regex,
    width: usize,
    height: usize,
    vector: Vec<Vector>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        for cap in self.regex.captures_iter(line) {
            let start = Point {
                x: cap.get(1).expect("no distance").as_str().parse::<usize>().expect("cannot parse"),
                y: cap.get(2).expect("no distance").as_str().parse::<usize>().expect("cannot parse"),
            };

            let end = Point {
                x: cap.get(3).expect("no distance").as_str().parse::<usize>().expect("cannot parse"),
                y: cap.get(4).expect("no distance").as_str().parse::<usize>().expect("cannot parse"),
            };

            self.width = self.width.max(start.x);
            self.width = self.width.max(end.x);

            self.height = self.height.max(start.y);
            self.height = self.height.max(end.y);

            self.vector.push(Vector {
                start,
                end,
            })
        }
    }
}

#[derive(Debug)]
struct Vector {
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            regex: Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap(),
            width: 0,
            height: 0,
            vector: vec![],
        }
    }
}

impl Container {
    fn calculate_1(&self) -> usize {
        let grid = self.get_grid();

        Self::calculate_pountos(grid)
    }

    fn calculate_2(&self) -> usize {
        let mut grid = self.get_grid();

        for vec in self.vector.iter().filter(|v| v.start.x != v.end.x && v.start.y != v.end.y) {
            let mut i = 0;
            while i <= vec.start.y.max(vec.end.y) - vec.start.y.min(vec.end.y) {
                let move_x: i32 = if vec.start.x < vec.end.x { 1 } else { -1 } * i as i32;
                let move_y: i32 = if vec.start.y < vec.end.y { 1 } else { -1 } * i as i32;

                let x = (vec.start.x as i32 + move_x) as usize;
                let y = (vec.start.y as i32 + move_y) as usize;
                grid[x][y] += 1;
                i += 1;
            }
        }

        Self::calculate_pountos(grid)
    }

    fn calculate_pountos(grid: Vec<Vec<usize>>) -> usize {
        let mut nb_2 = 0;

        for row in grid.iter() {
            for val in row.iter() {
                if *val >= 2 {
                    nb_2 += 1;
                }
            }
        }

        nb_2
    }

    fn get_grid(&self) -> Vec<Vec<usize>> {
        let mut grid: Vec<Vec<usize>> = vec![vec![0; self.height + 1]; self.width + 1];

        for vec in self.vector.iter().filter(|v| v.start.x == v.end.x) {
            let mut i = vec.start.y.min(vec.end.y);
            while i <= vec.start.y.max(vec.end.y) {
                grid[vec.start.x][i] += 1;
                i += 1;
            }
        }
        for vec in self.vector.iter().filter(|v| v.start.y == v.end.y) {
            let mut i = vec.start.x.min(vec.end.x);
            while i <= vec.start.x.max(vec.end.x) {
                grid[i][vec.start.y] += 1;
                i += 1;
            }
        }
        grid
    }
}