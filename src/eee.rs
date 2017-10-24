use std::io::{BufRead, Write};

pub type time = i64;

pub struct Simulator {}

impl Simulator {
    pub fn new(hyst: time, idle: time, input: &mut BufRead, output: &mut Write, log: &mut Write) {}
}
