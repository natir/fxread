use std::io::BufRead;
use anyhow::Result;
use super::fastx::FastxRead;
use super::record::Record;

pub struct FastaReader <R: BufRead> {
    reader: R,
    buffer: String
}
impl <R: BufRead> FastaReader <R> {
    pub fn new(reader: R) -> Self {
        Self { 
            reader,
            buffer: String::new()
        }
    }

    fn next_line(&mut self) -> Result<bool> {
        Ok(self.reader.read_line(&mut self.buffer)? > 0)
    }

    fn strip_header(&self, token: &str) -> String {
        token
            .trim_start_matches('>')
            .trim_end()
            .to_string()
    }

    fn strip_sequence(&self, token: &str) -> String {
        token
            .trim_end()
            .to_string()
    }
}

impl <R: BufRead> FastxRead for FastaReader<R> {

    fn next_record(&mut self) -> Result<Option<Record>> {
        let mut record = Record::new();

        for idx in 0..2 {
            self.buffer.clear();
            if !self.next_line()? { break }
            match idx {
                0 => {
                    record.set_id(self.strip_header(&self.buffer))
                },
                _ => {
                    record.set_seq(self.strip_sequence(&self.buffer))
                }
            }
        }

        if record.empty() {
            Ok(None)
        }
        else {
            Ok(Some(record))
        }
    }
}

impl <R: BufRead> Iterator for FastaReader <R> {

    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_record() {
            Ok(r) => r,
            Err(_) => panic!("Unexpected file end")
        }
    }

}
