use helper::InputReader;

fn main() {
    let mut container = Container::default();
    container.read("./input.txt").unwrap();

    println!("result 1 : {}", calculate(container.get_data()));

    let windows = container.get_data_window();
    let datas: Vec<Data> = windows.iter().map(|w| w.convert_to_data()).collect();

    println!("result 2 : {}", calculate(datas));
}

#[derive(Clone, Default)]
struct Container {
    datas: Vec<Data>,
    data_windows: Vec<DataWindow>,
}

impl Container {
    fn get_data(&self) -> Vec<Data> {
        self.datas.clone()
    }
    fn get_data_window(&self) -> Vec<DataWindow> {
        self.data_windows.clone()
    }
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        self.datas.push(Data {
            int: line.parse::<usize>().unwrap(),
        });

        if self.datas.len() >= 3 {
            self.data_windows.push(DataWindow {
                a: self.datas[self.datas.len() - 3].int,
                b: self.datas[self.datas.len() - 2].int,
                c: self.datas[self.datas.len() - 1].int,
            })
        }
    }
}

#[derive(Clone, Copy)]
struct Data {
    int: usize,
}

#[derive(Clone, Copy)]
struct DataWindow {
    a: usize,
    b: usize,
    c: usize,
}

impl DataWindow {
    fn convert_to_data(&self) -> Data {
        Data {
            int: self.a + self.b + self.c,
        }
    }
}

fn calculate(datas: Vec<Data>) -> usize {
    let mut previous: Option<Data> = None;
    let mut nb_increase = 0;

    for current in datas.iter() {
        match previous {
            None => {}
            Some(p) => {
                if current.int > p.int {
                    nb_increase += 1
                }
            }
        }
        previous = Some(*current);
    }

    nb_increase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let mut container = Container::default();
        container.read("./test.txt").unwrap();

        assert_eq!(calculate(container.get_data()), 7);

        let windows = container.get_data_window();
        let datas = windows.iter().map(|w| w.convert_to_data()).collect();
        assert_eq!(calculate(datas), 5);
    }
}
