use std::collections::HashMap;
use helper::InputReader;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths() {
        let mut container1 = Container::default();
        container1.read("./t1.txt").unwrap();

        let mut res1 = ContainerRes::default();
        res1.read("./r1.txt").unwrap();

        assert_eq!(sort_paths(container1.get_path(1)), sort_paths(res1.paths));

        let mut container2 = Container::default();
        container2.read("./t2.txt").unwrap();

        let mut res2 = ContainerRes::default();
        res2.read("./r2.txt").unwrap();

        assert_eq!(sort_paths(container2.get_path(1)), sort_paths(res2.paths));
    }

    #[test]
    fn test_paths_2() {
        let mut container1 = Container::default();
        container1.read("./t1.txt").unwrap();

        let mut res1 = ContainerRes::default();
        res1.read("./r1-2.txt").unwrap();

        assert_eq!(sort_paths(container1.get_path(2)), sort_paths(res1.paths));

        let mut container2 = Container::default();
        container2.read("./t2.txt").unwrap();

        assert_eq!(container2.get_path(2).len(), 103);
    }


    fn sort_paths(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut res = paths.clone();
        res.sort();
        res
    }

    #[test]
    fn test_calculate() {
        let mut container = Container::default();

        container.read("./test.txt").unwrap();


        assert_eq!(container.calculate_1(), 226);

        assert_eq!(container.calculate_2(), 3509);
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
    paths: HashMap<String, Vec<String>>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        let r: Vec<&str> = line.split("-").into_iter().collect();
        if r.len() != 2 {
            panic!("length is {}", r.len());
        }

        let (a, b) = (r[0], r[1]);
        self.append(a, b);
        self.append(b, a);
    }
}

impl Container {
    fn append(&mut self, s: &str, e: &str) {
        if s == "end" {
            return;
        }
        if e == "start" {
            return;
        }
        let paths = self.paths.entry(s.to_string()).or_insert(Vec::new());
        paths.push(e.to_string());
    }

    fn calculate_1(&self) -> isize {
        self.get_path(1).len() as isize
    }
    fn calculate_2(&self) -> isize {
        self.get_path(2).len() as isize
    }

    fn get_path(&self, exercice: usize) -> Vec<Vec<String>> {
        let mut all_path = Vec::new();

        let mut current_paths = self.get_one_more(vec!("start".to_string()), exercice);

        loop {
            let mut next_paths: Vec<Vec<String>> = Vec::new();
            for path in current_paths {
                for new in self.get_one_more(path, exercice) {
                    if !new.is_empty() {
                        if new.last().unwrap() == "end" {
                            all_path.push(new.clone());
                        } else {
                            next_paths.push(new);
                        }
                    }
                }
            }

            if next_paths.is_empty() {
                break;
            }
            current_paths = next_paths;
        }


        all_path
    }

    fn get_one_more(&self, current: Vec<String>, exercice: usize) -> Vec<Vec<String>> {
        let mut res = Vec::new();

        let next_nodes = self.paths.get(current.last().unwrap()).unwrap();

        for next in next_nodes.into_iter() {
            if next.to_ascii_lowercase() == *next {
                if current.contains(next) {
                    if exercice == 1 {
                        continue;
                    }else{
                        let mut to_test = current.clone();
                        to_test = to_test.into_iter().filter(|p| p.to_ascii_lowercase() == *p).collect();
                        to_test.sort();
                        let has_double:Vec<bool> = to_test.windows(2).into_iter().map(|a| a[0]== a[1] ).collect();
                        if has_double.contains(&true){
                            continue;
                        }
                    }
                }
            }

            let mut next_full = current.clone();
            next_full.push(next.clone());
            res.push(next_full);
        }

        res
    }
}

#[derive(Debug, Default)]
struct ContainerRes {
    paths: Vec<Vec<String>>,
}

impl InputReader for ContainerRes {
    fn add_line(&mut self, line: &str) {
        let r: Vec<String> = line.split(",").into_iter().map(|v| v.to_string()).collect();
        self.paths.push(r);
    }
}
