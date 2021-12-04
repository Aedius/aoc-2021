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
            4512
        );

        assert_eq!(
            container.calculate_2(),
            1924
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

#[derive(Default, Debug)]
struct Container {
    pos: Option<usize>,

    number: Vec<usize>,
    grid: Vec<Grid>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        match self.pos {
            None => {
                self.number = line.split(',').into_iter().map(|v| v.parse::<usize>().unwrap()).collect();
                self.pos = Some(0);
            }
            Some(pos) => {
                let new_line_vec:Vec<usize> = line.split(" ").into_iter()
                    .filter(|v| !v.is_empty())
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect();

                let  new_line=   new_line_vec.try_into().unwrap_or_else(|v: Vec<usize>| panic!("Expected a Vec of length {} but it was {}", 5, v.len()));


                if pos == 0 {
                    let mut new_grid = [[0; 5]; 5];

                    new_grid[0] = new_line;

                    self.grid.push(Grid{data: new_grid});
                } else {
                    let last_grid = self.grid.last_mut().unwrap();
                    last_grid.data[pos] = new_line;
                }


                if pos == 4 {
                    self.pos = Some(0);
                } else {
                    self.pos = Some(pos + 1);
                }
            }
        }
    }
}

impl Container {
    fn calculate_1(&self) -> usize {

        let mut pountos = 0;
        let mut numbers = self.number.clone();

        for grid in &self.grid{
            let (new_win, new_pountos) = grid.try_win(numbers.clone());

            match new_win{
                None => {}
                Some(win) => {
                    pountos = new_pountos * numbers[win];
                    numbers.truncate(win);
                }
            }
        }

        pountos

    }
    fn calculate_2(&self) -> usize {
        let mut pountos = 0;

        let mut worst_win = 0;

        for grid in &self.grid{
            let (new_win, new_pountos) = grid.try_win( self.number.clone());

            match new_win{
                None => {}
                Some(win) => {
                    if win > worst_win{
                        worst_win = win;
                        pountos = new_pountos * self.number[win];

                    }
                }
            }
        }

        pountos
    }
}

#[derive( Debug)]
struct Grid{
    data :[[usize; 5]; 5],
}

impl Grid{

    fn try_win(&self, numbers :Vec<usize>) -> (Option<usize>, usize){

        let mut row_nb = [0; 5];
        let mut col_nb = [0; 5];

        let mut data = self.data.clone();

        for (index, number) in numbers.iter().enumerate() {
            for (i, row) in self.data.iter().enumerate() {
                for (j, value) in row.iter().enumerate() {
                    if *number == *value{
                        row_nb[i] += 1;
                        col_nb[j] += 1;

                        data[i][j] = 0;

                        if row_nb.contains(&5) || col_nb.contains(&5){

                            return (Some(index), sum(data));

                        }

                    }
                }
            }
        }


        (None, 0)
    }

}

fn sum ( d :[[usize; 5]; 5]) -> usize{
    let mut r = 0;
    for  row in d.iter() {
        for  value in row.iter() {
            r += *value
        }
    }
    r
}