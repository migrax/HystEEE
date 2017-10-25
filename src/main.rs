#![feature(slice_patterns)]
#[macro_use]

extern crate clap;
extern crate eee_hyst;

use clap::App;
use std::fs::File;
use std::io;
use std::io::{Read, BufReader, BufRead, BufWriter, Write};
use eee_hyst::{Time, simulator};
use eee_hyst::switch::Packet;
use std::iter::Iterator;

struct PacketsFromRead<R: Read> {
    is: BufReader<R>,
}

impl<R: Read> PacketsFromRead<R> {
    pub fn new(buf: R) -> PacketsFromRead<R> {
        PacketsFromRead { is: BufReader::new(buf) }
    }
}

impl<R: Read> Iterator for PacketsFromRead<R> {
    type Item = Packet;

    fn next(&mut self) -> Option<Packet> {
        let line = &mut String::new();

        let entries: Vec<u64> = match self.is.read_line(line) {
            Err(_) => return None,
            _ => {
                line.split_whitespace()
                    .map(|x| x.parse().expect("Could not parse input as a number."))
                    .collect()
            }
        };

        match &entries[..] {
            &[arrival, size] => Some(Packet::new(Time(arrival), size as u32)),
            _ => None,
        }
    }
}

fn main() {
    let yaml = load_yaml!("eee.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let hyst = matches.value_of("hyst").unwrap().parse();
    let maxidle = matches.value_of("delay").unwrap().parse();

    if hyst.is_err() || maxidle.is_err() {
        eprintln!("Could parse a number.");
        ::std::process::exit(1);
    }

    let stdin = io::stdin();
    let input_read;

    match matches.value_of("INPUT") {
        Some(filename) => {
            let file = File::open(filename);
            if !file.is_ok() {
                eprintln!("Could not open input file {}.", filename);
                ::std::process::exit(1);
            }
            input_read = Box::new(BufReader::new(file.unwrap())) as Box<BufRead>;
        }
        None => input_read = Box::new(stdin.lock()) as Box<BufRead>,
    }

    let mut trace_writer;
    match matches.value_of("OUTPUT") {
        Some(filename) => {
            let file = File::create(filename);
            if !file.is_ok() {
                eprintln!("Could not open trace file {} for writing.", filename);
                ::std::process::exit(2);
            }
            trace_writer = BufWriter::new(Box::new(BufWriter::new(file.unwrap())) as Box<Write>);
        }
        None => trace_writer = BufWriter::new(Box::new(io::stdout()) as Box<Write>),
    }

    let mut log_writer;
    match matches.value_of("LOG") {
        Some(filename) => {
            let file = File::create(filename);
            if !file.is_ok() {
                eprintln!("Could not open log file {} for writing.", filename);
                ::std::process::exit(2);
            }
            log_writer = Some(BufWriter::new(file.unwrap()));
        }
        None => log_writer = None,
    }

    let simul = simulator::Simulator::new(
        hyst.expect("Hystereris was not a proper number."),
        maxidle.expect("Delay was not a properly formatted number."),
        PacketsFromRead::new(input_read),
    );

    for ev in simul
        .map(|ev| match ev {
            (time, Some(packet), _) => {
                writeln!(trace_writer, "{}\t{}", time, packet.size())
                    .expect("Error writing output trace.");
                ev
            }
            (_, None, _) => ev,
        })
        .filter(|ev| match *ev {
            (_, _, Some(_)) => true,
            _ => false,
        })
    {
        if log_writer.is_some() {
            match ev {
                (time, _, Some(ref status)) => {
                    writeln!(log_writer.as_mut().unwrap(), "{}\t{}", time, status)
                        .expect("Error writing output log.");
                }
                _ => panic!("No such event in here"),
            }
        }
    }
}
