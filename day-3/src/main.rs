use helper::InputReader;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let mut container = Container::default();
        container.read("./test.txt").unwrap();

        assert_eq!(
            container.calculate_1(),
            Result1 {
                gamma: 22,
                epsilon: 9
            }
        );

        assert_eq!(
            container.calculate_2(),
            Result2 {
                oxygen: 23,
                co2: 10
            }
        );
    }
}

fn main() {
    let mut container = Container::default();
    container.read("./input.txt").unwrap();

    let res = container.calculate_1();

    println!("result 1 : {} --- {:?}", res.gamma * res.epsilon, res);

    let res = container.calculate_2();

    println!("result 2 : {} --- {:?}", res.oxygen * res.co2, res);
}

#[derive(Default)]
struct Container {
    data: Vec<Vec<Bit>>,
}

#[derive(PartialEq, Debug)]
enum Bit {
    Zero,
    One,
}

#[derive(PartialEq, Debug)]
struct Result1 {
    gamma: usize,
    epsilon: usize,
}

#[derive(PartialEq, Debug)]
struct Result2 {
    oxygen: usize,
    co2: usize,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        let mut bitlist = Vec::new();
        for c in line.chars() {
            if c == '0' {
                bitlist.push(Bit::Zero)
            } else {
                bitlist.push(Bit::One)
            }
        }

        self.data.push(bitlist);
    }
}

impl Container {
    fn calculate_1(&self) -> Result1 {
        let mut gamma = 0;
        let mut epsilon = 0;

        let max_index = self.data[0].len() - 1;

        let mut i = 0;

        while let Some(most) = get_most(&self.data, i) {
            if most == Bit::One {
                gamma += 2_usize.pow((max_index - i) as u32);
            } else {
                epsilon += 2_usize.pow((max_index - i) as u32);
            }
            i += 1;
        }

        Result1 { gamma, epsilon }
    }

    fn calculate_2(&self) -> Result2 {
        let res = filter_most(&self.data, 0, true);
        let oxygen = Self::get_int(&res[0]);

        let res = filter_most(&self.data, 0, false);
        let co2 = Self::get_int(&res[0]);

        Result2 { oxygen, co2 }
    }

    fn get_int(data: &[Bit]) -> usize {
        let max_index = data.len() - 1;
        let mut value = 0;

        for (i, bit) in data.iter().enumerate() {
            if *bit == Bit::One {
                value += 2_usize.pow((max_index - i) as u32);
            }
        }
        value
    }
}

fn filter_most(row_list: &[Vec<Bit>], position: usize, is_most: bool) -> Vec<Vec<Bit>> {
    let filter;
    if is_most {
        filter = get_most(row_list, position);
    } else {
        filter = get_fewer(row_list, position);
    }

    let mut res = Vec::new();

    for row in row_list.iter() {
        match filter {
            None => {
                res.push(copy_row(row));
            }
            Some(Bit::One) => {
                if row[position] == Bit::One {
                    res.push(copy_row(row));
                }
            }
            Some(Bit::Zero) => {
                if row[position] == Bit::Zero {
                    res.push(copy_row(row));
                }
            }
        }
    }

    let stop_filter = res.len() == 1 || filter.is_none();

    if stop_filter {
        return res;
    }

    filter_most(&res, position + 1, is_most)
}

fn copy_row(row: &[Bit]) -> Vec<Bit> {
    let mut bitlist = Vec::new();
    for bit in row.iter() {
        let b: Bit;
        if *bit == Bit::One {
            b = Bit::One;
        } else {
            b = Bit::Zero;
        }
        bitlist.push(b);
    }
    bitlist
}

fn get_most(row_list: &[Vec<Bit>], position: usize) -> Option<Bit> {
    let mut nb_one = 0;
    let mut nb_zero = 0;

    let mut found = false;

    for row in row_list.iter() {
        for (i, bit) in row.iter().enumerate() {
            if i == position {
                found = true;
                if *bit == Bit::One {
                    nb_one += 1
                } else {
                    nb_zero += 1
                }
            }
        }
    }

    if !found {
        return None;
    }

    if nb_one >= nb_zero {
        Some(Bit::One)
    } else {
        Some(Bit::Zero)
    }
}

fn get_fewer(row_list: &[Vec<Bit>], position: usize) -> Option<Bit> {
    match get_most(row_list, position) {
        None => None,
        Some(Bit::One) => Some(Bit::Zero),
        Some(Bit::Zero) => Some(Bit::One),
    }
}
