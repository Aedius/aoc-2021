use std::fmt::{Debug};
use helper::InputReader;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterate() {
        let mut container = Container::default();
        container.read("./test.txt").unwrap();

        let mut container1 = Container::default();
        container1.read("./test-r1.txt").unwrap();
        assert_eq!(iterate(container.map), (container1.map, 0));

        let mut container2 = Container::default();
        container2.read("./test-r2.txt").unwrap();
        assert_eq!(iterate(container1.map), (container2.map, 35));

        let mut previous_container = container2;
        for i in 3..11{
            let mut local_container = Container::default();
            local_container.read(format!("./test-r{}.txt", i).as_str()).unwrap();
            assert_eq!(iterate(previous_container.map).0, local_container.map, "failed for test-r{}.txt", i);

            previous_container = local_container;

        }

        for i in (20..101).step_by(10){
            let mut local_container = Container::default();
            local_container.read(format!("./test-r{}.txt", i).as_str()).unwrap();
            assert_eq!(iterate_nb(previous_container.map, 10).0, local_container.map, "failed for test-r{}.txt", i);

            previous_container = local_container;
        }

    }

    #[test]
    fn test_calculate() {
        let mut container = Container::default();
        container.read("./test.txt").unwrap();

        assert_eq!(container.calculate_1(), 1656);

        assert_eq!(container.calculate_2(), 195);
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
    map: Map,
    cursor: usize,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        for (i, c) in line.chars().enumerate() {
            self.map[self.cursor+1][i+1] = c.to_string().parse::<isize>().unwrap()
        }
        self.cursor += 1;
    }
}

impl Container {
    fn calculate_1(&self) -> isize {
        let res = iterate_nb(self.map, 100);
        res.1

    }
    fn calculate_2(&self) -> isize {
        let mut nb = 0;
        let mut map = self.map;
        let sync = Map::default();
        while map != sync{
            let i = iterate(map);
            map = i.0;
            nb+=1;
        }

        nb
    }
}

type Map = [[isize; 12]; 12];

fn iterate_nb(map: Map, nb : isize) -> (Map, isize) {
    let mut res = map;
    let mut exp = 0;
    for _i in 0..nb{
        let t = iterate(res);
        res = t.0;
        exp +=t.1;
    }
    (res, exp)
}

fn iterate(map: Map) -> (Map, isize) {

    let mut explosion = Vec::new();
    let mut nb = 0;
    let mut res = Map::default();

    for i in 1..11 {
        for j in 1..11 {
            let value = map[i][j] + 1;
            if value == 10 {
                nb += 1;
                explosion.push((i, j))
            }
            res[i][j] = value;
        }
    }

    loop {
        let mut current_explosion = Vec::new();

        for (i, j) in &explosion{
            for k in i-1..i+2{
                for l in j-1..j+2{

                    let value = res[k][l] + 1;

                    if value == 10 {
                        nb += 1;
                        current_explosion.push((k, l))
                    }
                    res[k][l] = value;
                }
            }
        }

        res = reset_bord(res);

        if current_explosion.is_empty(){
            break;
        }
        explosion = current_explosion;
    }

    res = cleanup_exp(res);

    display(&res);

    (res, nb)
}

fn reset_bord(mut map: Map) -> Map{
    for i in 0..12 {
        for j in 0..12 {
            if i == 0 || i == 11 || j == 0 || j == 11{
                map[i][j] = 0;
            }
        }
    }
    map
}

fn cleanup_exp(mut map: Map) -> Map{
    for i in 0..12 {
        for j in 0..12 {
            if  map[i][j]>9 {
                map[i][j] = 0;
            }
        }
    }
    map
}

fn display(map: &Map){
    for i in 1..11 {
        for j in 1..11 {
            print!("{:3}",map[i][j]);
        }
        println!();
    }
    println!();
}