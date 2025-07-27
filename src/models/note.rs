use std::path::PathBuf;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub struct Note {
    pub name: String,
    pub path: PathBuf,
    pub lines: Option<Vec<String>>
}

impl Note {
    pub fn from(name: String, path: PathBuf) -> Self {
        Note {
            name,
            path,
            lines: None,
        }
    }

    pub fn read_contents(&mut self) -> Result<&Vec<String>, Error> {
        let mut file_lines: Vec<String> = vec![];
        match std::fs::File::open(&self.path) {
            Ok(file) => {
                let bufreader = BufReader::new(file).lines();
                for line in bufreader {
                    file_lines.push(line.unwrap_or(String::from("")));
                }

                self.lines = Some(file_lines);
                Ok(&self.lines.as_ref().unwrap())
            }
            _ => {
                Err(Error::new(ErrorKind::NotFound, format!("failed to open note {}", self.name)))
            }
        }
    }

    pub fn try_get_lines(&mut self) -> Option<&Vec<String>> {
        if self.lines == None {
            match self.read_contents() {
                Ok(lines) => return Some(lines),
                Err(_) => return None,
            }
        }

        Some(&self.lines.as_ref().unwrap())
    }
}
