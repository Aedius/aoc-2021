use helper::InputReader;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let mut container = Container::default();
        container.read("./test.txt").unwrap();

        assert_eq!(container.calculate_1(), 15);

        let mut container_input = Container::default();
        container_input.read("./input.txt").unwrap();

        assert_ne!(container_input.calculate_1(), 1508);

        assert_eq!(container.calculate_2(), 1134);
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
    map: Vec<Vec<u32>>,
    w: usize,
    h: usize,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        const RADIX: u32 = 10;

        let row = line.chars().into_iter().map(|c| c.to_digit(RADIX).unwrap()).collect();

        self.map.push(row)
    }
}

impl Container {
    fn calculate_1(&self) -> isize {
        let res = self.get_low_points();


        res.into_iter().map(|t| t.v + 1).sum()
    }

    fn get_low_points(&self) -> Vec<Point> {
        let mut res: Vec<Point> = Vec::new();

        for (i, l) in self.map.clone().into_iter().enumerate() {
            for (j, v) in l.clone().into_iter().enumerate() {
                let mut is_smallest = true;
                if j > 0 {
                    if l[j - 1] <= v {
                        is_smallest = false;
                    }
                }
                if j < l.len() - 1 {
                    if l[j + 1] <= v {
                        is_smallest = false;
                    }
                }
                if i > 0 {
                    if self.map[i - 1][j] <= v {
                        is_smallest = false;
                    }
                }
                if i < self.map.len() - 1 {
                    if self.map[i + 1][j] <= v {
                        is_smallest = false;
                    }
                }

                if is_smallest {
                    res.push(Point {
                        x: i,
                        y: j,
                        v: v as isize,
                    });
                }
            }
        }
        res
    }

    fn calculate_2(&mut self) -> isize {
        let points = self.get_low_points();
        let mut res: Vec<Bassin> = Vec::new();

        self.w = self.map.len() - 1;
        self.h = self.map[0].len() - 1;

        for lowest in points.into_iter() {
            let mut bassin = Bassin {
                list: vec![lowest]
            };
            let mut iteration = vec![lowest];

            while let Some(next_iteration) = self.get_next_generation(&iteration) {
                iteration.clear();
                for p in next_iteration {
                    if !bassin.list.contains(&p) {
                        bassin.list.push(p);
                        iteration.push(p);
                    }
                }
            }


            res.push(bassin);
        }


        res.sort_by(|a, b| a.list.len().partial_cmp(&b.list.len()).unwrap());
        let mut res: Vec<isize> = res.into_iter().rev().map(|b| b.list.len() as isize).collect();
        res.truncate(3);
        res.into_iter().product()
    }

    fn get_next_generation(&mut self, iteration: &Vec<Point>) -> Option<Vec<Point>> {
        let mut next_iteration = vec![];
        for p in iteration.clone().into_iter() {
            if p.x > 0 {
                if self.map[p.x - 1][p.y] != 9 {
                    next_iteration.push(Point {
                        x: p.x - 1,
                        y: p.y,
                        v: self.map[p.x - 1][p.y] as isize,
                    });
                }
            }
            if p.x < self.w {
                if self.map[p.x + 1][p.y] != 9 {
                    next_iteration.push(Point {
                        x: p.x + 1,
                        y: p.y,
                        v: self.map[p.x + 1][p.y] as isize,
                    });
                }
            }
            if p.y > 0 {
                if self.map[p.x][p.y - 1] != 9 {
                    next_iteration.push(Point {
                        x: p.x,
                        y: p.y - 1,
                        v: self.map[p.x][p.y - 1] as isize,
                    });
                }
            }
            if p.y < self.h {
                if self.map[p.x][p.y + 1] != 9 {
                    next_iteration.push(Point {
                        x: p.x,
                        y: p.y + 1,
                        v: self.map[p.x][p.y + 1] as isize,
                    });
                }
            }
        }

        if next_iteration.is_empty() {
            None
        } else {
            Some(next_iteration)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    v: isize,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Bassin {
    list: Vec<Point>,
}