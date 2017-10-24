#[macro_use]
extern crate clap;
extern crate eee_hyst;

use clap::App;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead, BufWriter, Write};
use eee_hyst::simulator;

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
    let mut input_read;

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

    let mut simul = simulator::Simulator::new(
        hyst.expect("Hystereris was not a proper number."),
        maxidle.expect("Delay was not a properly formatted number."),
        &mut input_read,
        &mut trace_writer,
        log_writer.as_mut(),
    );

    ::std::process::exit(match simul.run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("Error during simulation: {}.", err);
            1
        }
    });
}
