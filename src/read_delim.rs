use std::io::prelude::*;
use std::fs::File;

pub struct ReadDelimiter {
    pub _filename: String,
    pub file: Option<File>,
    pub delimiter: String,
    pub line: String, 
}

impl ReadDelimiter {

    pub fn new(filename: String, delimiter: String) -> Result<ReadDelimiter, std::io::Error>{
        let file = File::open(&filename)?;
        let res = ReadDelimiter {
            _filename: filename.clone(),
            file: Some(file),
            delimiter: delimiter,
            line: "".to_string(),
        };
        Ok(res)
    }

    pub fn read(&mut self) -> Result<bool, std::io::Error> {
        let file = match self.file.as_mut() {
            Some(file) => file, // Borrow file mutably
            None => return Ok(false), // No file, return false
        };

        self.line = "".to_string();
        let mut buffer: [u8; 1] = [0; 1];
        let mut check_delim: usize = 0;
        let mut save_check_delim: String = "".to_string();

        while let Ok(bytes_read) = file.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            if buffer[0] == self.delimiter.as_bytes()[check_delim] 
                && ((check_delim + 1) as usize) == self.delimiter.len(){
                break;
            }
            else if buffer[0] == self.delimiter.as_bytes()[check_delim] {
                check_delim+=1;
                save_check_delim += &((buffer[0] as char).to_string());
                continue;
            }
            if check_delim != 0 {
                self.line += &(save_check_delim.to_string());
                save_check_delim = "".to_string();
                check_delim = 0;
            }
            self.line += &((buffer[0] as char).to_string());
        }
    
        Ok(self.line.len() != 0)
    }
}