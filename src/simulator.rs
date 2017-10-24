use std::io::{BufRead, BufWriter, Write};
use std::fs::File;

pub type Time = i64;

pub struct Simulator<'a> {
    hyst: Time,
    idle: Time,
    input: &'a mut BufRead,
    output: &'a mut Write,
    log: Option<&'a mut BufWriter<File>>,
}

impl<'a> Simulator<'a> {
    pub fn new(
        hyst: Time,
        idle: Time,
        input: &'a mut BufRead,
        output: &'a mut Write,
        log: Option<&'a mut BufWriter<File>>,
    ) -> Simulator<'a> {
        Simulator {
            hyst: hyst,
            idle: idle,
            input: input,
            output: output,
            log: log,
        }
    }

    pub fn run(&mut self) -> Result<i32, &str> {
        Ok(0)
    }
}
