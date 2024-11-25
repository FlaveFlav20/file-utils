use std::fs::File;
use std::io::prelude::*;

use crate::utils::non_ascii_char::non_ascii_char;

pub struct ReadDelimiter {
    pub _filename: String,
    pub file: File,
    pub delimiter: Vec<String>,
    pub line: String,
    buffer: Vec<u8>,
    index_buffer: usize,
    curr_index: usize,
}
/*
    ReadDelimiter:
        - Goal: Create a structure to read a file delim by delim (like line by line)
*/

impl ReadDelimiter {
    pub fn new(
        filename: String,
        delimiter: Vec<String>,
        buffer_size: usize,
    ) -> Result<ReadDelimiter, std::io::Error> {
        let file = File::open(&filename)?;
        Ok(ReadDelimiter {
            _filename: filename.clone(),
            file: file,
            delimiter: delimiter.clone(),
            line: "".to_string(),
            buffer: vec![0; buffer_size],
            index_buffer: 0,
            curr_index: 0,
        })
    }

    fn read_non_ascii_char(&mut self, first_u8: u8) -> Result<(), std::io::Error> {
        let check_size: i8 = non_ascii_char::check_number_bytes_begin(first_u8);
        if check_size <= 0 {
            self.line.push('�');
            println!("Not a valid character!");
            return Ok(());
        }
        let size: usize = check_size as usize;
        let mut chars: Vec<u8> = Vec::new();
        chars.push(first_u8);

        let mut buffer: u8 = 0;

        for _ in 1..size {
            let bytes_read = match self.read_from_buffer(&mut buffer) {
                Ok(bytes_read) => bytes_read,
                Err(e) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!(
                            "[ReadDelimiter][read_non_ascii_char]: Error reading file: {}",
                            e
                        ),
                    ));
                }
            };

            if bytes_read == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "Unexpected EOF while reading multi-byte character",
                ));
            }

            // TO DO: check if invalid ascii follow

            chars.push(buffer);
        }
        if let Ok(valid_str) = std::str::from_utf8(&chars) {
            self.line.push_str(valid_str);
        } else {
            self.line.push('�');
        }
        Ok(())
    }

    pub fn read(&mut self) -> Result<bool, std::io::Error> {
        self.line = "".to_string();
        let mut buffer: u8 = 0;

        while let Ok(bytes_read) = self.read_from_buffer(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            if non_ascii_char::check_non_ascii(buffer) {
                let _ = self.read_non_ascii_char(buffer);
            } else {
                self.line += &(buffer as char).to_string();
            }

            for i in 0..self.delimiter.len() {
                if self.delimiter[i].as_bytes().len() == 0 {
                    continue;
                }
                if self.line.len() < self.delimiter[i].as_bytes().len() {
                    continue;
                }

                let str = self
                    .line
                    .get(self.line.len() - self.delimiter[i].len()..)
                    .unwrap_or("");

                if self.delimiter[i] == str {
                    for _i in 0..self.delimiter[i].chars().count() {
                        self.line.pop();
                    }
                    return Ok(true);
                }
            }
        }
        Ok(self.line.len() != 0)
    }

    fn read_from_buffer(&mut self, c: &mut u8) -> Result<usize, std::io::Error> {
        if self.curr_index >= self.index_buffer {
            let bytes_read = match (self.file).read(&mut self.buffer) {
                Ok(bytes_read) => bytes_read,
                Err(_e) => panic!("[ReadDeliiter][read_from_buffer]: Error while reading file"),
            };

            if bytes_read == 0 {
                return Ok(0);
            }

            self.curr_index = 0;
            self.index_buffer = bytes_read;
        }
        *c = self.buffer[self.curr_index] as u8;
        self.curr_index += 1;
        return Ok(1 as usize);
    }
}
