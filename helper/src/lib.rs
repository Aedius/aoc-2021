use easy_reader::EasyReader;
use std::{fs::File, io::Error};

pub trait InputReader {
    fn read(&mut self, path: &str) -> Result<(), Error> {
        let file = File::open(path)?;

        let mut reader = EasyReader::new(file)?;

        while let Some(line) = reader.next_line()? {
            self.add_line(line.as_str());
        }

        Ok(())
    }

    fn add_line(&mut self, line: &str);
}
