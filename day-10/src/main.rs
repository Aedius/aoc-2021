use helper::InputReader;

#[cfg(test)]
mod tests {
    use crate::LineResult::Corrupted;
    use super::*;

    #[test]
    fn test_calculate() {
        let r = Container::parse("{([(<{}[<>[]}>{[]{[(<()>");
        assert_eq!(r.get_result(), Corrupted(Symbol::C));


        let r = Container::parse("[[<[([]))<([[{}[[()]]]");
        assert_eq!(r.get_result(), Corrupted(Symbol::A));


        let r = Container::parse("[{[{({}]{}}([{[{{{}}([]");
        assert_eq!(r.get_result(), Corrupted(Symbol::B));


        let r = Container::parse("[<(<(<(<{}))><([]([]()");
        assert_eq!(r.get_result(), Corrupted(Symbol::A));


        let r = Container::parse("<{([([[(<>()){}]>(<<{{");
        assert_eq!(r.get_result(), Corrupted(Symbol::D));


        let mut container = Container::default();
        container.read("./test.txt").unwrap();
        assert_eq!(container.calculate_1(), 26397);


        assert_eq!(container.calculate_2(), 288957);
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
    rows: Vec<SymList>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        let list = Self::parse(line);

        self.rows.push(list);
    }
}


impl Container {
    fn calculate_1(&self) -> isize {
        let mut pts = 0;

        for r in &self.rows {
            match r.get_result() {
                LineResult::Fine => {}
                LineResult::Incomplete(_) => {}
                LineResult::Corrupted(sym) => {
                    pts += sym.pts1();
                }
            }
        }

        pts
    }
    fn calculate_2(&self) -> isize {
        let mut scores = Vec::new();

        for r in &self.rows {
            match r.get_result() {
                LineResult::Fine => {}
                LineResult::Incomplete(missing) => {
                    let mut pts = 0;

                    for next in missing {
                        pts = pts * 5 + next.pts2();
                    }
                    scores.push(pts);
                }
                LineResult::Corrupted(_) => {}
            }
        }
        scores.sort();

        scores[scores.len() / 2]
    }

    fn parse(line: &str) -> SymList {
        let mut list = SymList {
            l: Vec::new()
        };

        for c in line.chars() {
            match c {
                '(' => list.l.push(Sym::In(Symbol::A)),
                ')' => list.l.push(Sym::Out(Symbol::A)),
                '[' => list.l.push(Sym::In(Symbol::B)),
                ']' => list.l.push(Sym::Out(Symbol::B)),
                '{' => list.l.push(Sym::In(Symbol::C)),
                '}' => list.l.push(Sym::Out(Symbol::C)),
                '<' => list.l.push(Sym::In(Symbol::D)),
                '>' => list.l.push(Sym::Out(Symbol::D)),
                _ => {}
            }
        }
        list
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
enum Symbol {
    A,
    B,
    C,
    D,
}

impl Symbol {
    fn pts1(&self) -> isize {
        match &self {
            Symbol::A => { 3 }
            Symbol::B => { 57 }
            Symbol::C => { 1197 }
            Symbol::D => { 25137 }
        }
    }
    fn pts2(&self) -> isize {
        match &self {
            Symbol::A => { 1 }
            Symbol::B => { 2 }
            Symbol::C => { 3 }
            Symbol::D => { 4 }
        }
    }
}

#[derive(Debug)]
enum Sym {
    In(Symbol),
    Out(Symbol),
}

#[derive(Debug)]
struct SymList {
    l: Vec<Sym>,
}

#[derive(PartialEq, Debug)]
enum LineResult {
    Fine,
    Incomplete(Vec<Symbol>),
    Corrupted(Symbol),
}

impl SymList {
    fn get_result(&self) -> LineResult {
        let mut open = Vec::new();

        for s in &self.l {
            match s {
                Sym::In(si) => {
                    open.push(*si)
                }
                Sym::Out(so) => {
                    if open.is_empty() {
                        return LineResult::Corrupted(*so);
                    }
                    if so == open.last().unwrap() {
                        open.pop();
                    } else {
                        return LineResult::Corrupted(*so);
                    }
                }
            }
        }

        if open.is_empty() {
            LineResult::Fine
        } else {
            open.reverse();
            LineResult::Incomplete(open)
        }
    }
}