use std::io::prelude::*;
use std::fs::File;

pub struct ReadDelimiter {
    pub _filename: String,
    pub file: File,
    pub delimiter: Vec<String>,
    pub line: String, 
    buffer: Vec<u8>,
    index_buffer: usize,
    curr_index: usize,
    count: usize,
}

/*
    ReadDelimiter: 
        - Goal: Create a structure to read a file delim by delim (like line by line)
*/

impl ReadDelimiter {

    pub fn new(filename: String, delimiter: Vec<String>, buffer_size: usize) -> Result<ReadDelimiter, std::io::Error>{
        let file = File::open(&filename)?;
        Ok(ReadDelimiter {
            _filename: filename.clone(),
            file: file,
            delimiter: delimiter.clone(),
            line: "".to_string(),
            buffer: vec![0; buffer_size],
            index_buffer: 0,
            curr_index: 0,
            count: 0,
        })
    }

    pub fn read(&mut self) -> Result<bool, std::io::Error> {

        self.line = "".to_string();
        let mut buffer: u8 = 0;
        let mut indx: usize = 0;

        while let Ok(bytes_read) = self.read_from_buffer(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            self.line += &((buffer as char).to_string());

            for i in 0..self.delimiter.len() {
                if indx < (self.delimiter[i].as_bytes().len() - 1) {
                    continue;
                }

                if self.delimiter[i] == &self.line[((indx - (self.delimiter[i].as_bytes().len() - 1)))..self.line.len()] {
                    for _i in 0..self.delimiter[i].as_bytes().len() {
                        self.line.pop();
                    }
                    return Ok(true);
                }
            }
            indx+=1;
        }
        Ok(self.line.len() != 0)
    }

    fn read_from_buffer(&mut self, c: &mut u8) -> Result<usize, std::io::Error>{
        self.count += 1;
        if self.curr_index >= self.index_buffer {
    

            let bytes_read = match (self.file).read(&mut self.buffer) {
                Ok(bytes_read) => bytes_read,
                Err(_e) => panic!("[ReadDeliiter][read_from_buffer]: Error while reading file"),
            };

            if bytes_read == 0 {
                return Ok(0);
            }

            self.curr_index = 0;
            if self.index_buffer == 0 {
                self.index_buffer = 1023;
            }
        }
        *c = self.buffer[self.curr_index] as u8;
        self.curr_index += 1;
        return Ok(1 as usize);
    }

}