#[macro_use]

extern crate clap;

use clap::App;
use eee_hyst::switch::{Packet, Status};
use eee_hyst::{simulator, Time};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::iter::Iterator;

struct PacketsFromRead<'a, R: BufRead + ?Sized> {
    is: &'a mut R,
}

impl<'a, R: BufRead + ?Sized> PacketsFromRead<'a, R> {
    pub fn new(buf: &'a mut R) -> PacketsFromRead<'a, R> {
        PacketsFromRead { is: buf }
    }
}

impl<'a, R: BufRead + ?Sized> Iterator for PacketsFromRead<'a, R> {
    type Item = Packet;

    fn next(&mut self) -> Option<Packet> {
        let line = &mut String::new();

        match self.is.read_line(line) {
            Err(_) => None,
            _ => {
                let values: Vec<&str> = line.split_whitespace().collect();

                match values.len() {
                    0 => None, // Just an empty line
                    2 => Some(Packet::new(
                        Time::from_secs(values[0].parse().unwrap_or_else(|_| {
                            panic!("{} is not a valid arrival time.", values[0])
                        })),
                        values[1]
                            .parse()
                            .unwrap_or_else(|_| panic!("{} is not a valid size.", values[1])),
                    )),
                    _ => {
                        eprintln!("Malformed line \"{}\"", line);
                        ::std::process::exit(1)
                    }
                }
            }
        }
    }
}

struct Stats {
    last_state: Status,
    totals: HashMap<Status, Time>,
    total_time: Time,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            last_state: Status::Off,
            totals: HashMap::new(),
            total_time: Time(0),
        }
    }

    fn update(&mut self, info: (Time, Status)) {
        let (time, state) = info;
        let stats = self.totals.entry(self.last_state).or_insert(Time(0));
        self.last_state = state;
        *stats = (*stats + time) - self.total_time;
        self.total_time = time;
    }

    fn get_total_time(&self) -> Time {
        self.total_time
    }
}

impl<'a> IntoIterator for &'a mut Stats {
    type Item = (&'a Status, &'a Time);
    type IntoIter = std::collections::hash_map::Iter<'a, Status, Time>;

    fn into_iter(self) -> Self::IntoIter {
        self.totals.iter()
    }
}

fn main() {
    let yaml = load_yaml!("eee.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let verbose = matches.is_present("verbose");

    let hyst = matches.value_of("hyst").unwrap().parse();
    let maxidle = matches.value_of("delay").unwrap().parse();

    if hyst.is_err() || maxidle.is_err() {
        eprintln!("Could parse a number.");
        ::std::process::exit(1);
    }

    let stdin = io::stdin();
    let mut file_reader;
    let mut stdin_reader;

    let input_read: &mut dyn BufRead = match matches.value_of("INPUT") {
        Some(filename) => {
            let file = File::open(filename);
            if file.is_err() {
                eprintln!("Could not open input file {}.", filename);
                ::std::process::exit(1);
            }
            file_reader = BufReader::new(file.unwrap());
            &mut file_reader
        }
        None => {
            stdin_reader = stdin.lock();
            &mut stdin_reader
        }
    };

    let stdout = io::stdout();

    let mut trace_writer = match matches.value_of("OUTPUT") {
        Some(filename) => {
            let file = File::create(filename);
            if file.is_err() {
                eprintln!("Could not open trace file {} for writing.", filename);
                ::std::process::exit(2);
            }
            BufWriter::new(Box::new(file.unwrap()) as Box<dyn Write>)
        }
        None => BufWriter::new(Box::new(stdout.lock()) as Box<dyn Write>),
    };

    let mut log_writer;
    match matches.value_of("LOG") {
        Some(filename) => {
            let file = File::create(filename);
            if file.is_err() {
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

    let mut stats = Stats::new();
    for state in simul
        .map(|ev| match ev {
            (time, Some(packet), _) => {
                writeln!(trace_writer, "{:e}\t{}", time.as_secs(), packet.size())
                    .expect("Error writing output trace.");
                ev
            }
            (_, None, _) => ev,
        })
        .filter_map(|ev| match ev {
            (time, _, Some(state)) => Some((time, state)),
            _ => None,
        })
        .map(|ev| {
            if verbose && log_writer.is_some() {
                writeln!(
                    log_writer.as_mut().unwrap(),
                    "{:e}\t{}",
                    ev.0.as_secs(),
                    ev.1
                )
                .expect("Error writing output log.");
            }
            ev
        })
    {
        stats.update(state);
    }

    if log_writer.is_some() {
        let total = stats.get_total_time();
        for (state, time) in stats.into_iter() {
            writeln!(
                log_writer.as_mut().unwrap(),
                "#\t{}:\t{:e}s\t{:5.2}%",
                state,
                time.as_secs(),
                100.0 * (*time / total)
            )
            .expect("Error writing to output log.");
        }
    }
}
