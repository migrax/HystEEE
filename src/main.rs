use eee_hyst::switch::{Packet, Status};
use eee_hyst::{simulator, Time};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::iter::Iterator;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(author, about)]
struct Opt {
    /// Time before entering LPI in ns
    #[structopt(short = "h", long = "hyst", default_value = "0")]
    hyst: u64,

    /// Time since firt scheduled packet in LPI until resuming normal mode in ns
    #[structopt(short = "d", long = "delay", default_value = "0")]
    delay: u64,

    /// Traffic input file to use. Format "time (s) length (bytes)". Leaveeee empty for STDIN
    #[structopt(name = "INPUT", parse(from_os_str))]
    input: Option<PathBuf>,

    /// Traffic output file. Same format as INPUT. Uses stdout if not present.
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Log output filename, if present
    #[structopt(short = "l", long = "log", parse(from_os_str))]
    log: Option<PathBuf>,

    /// Write verbose log. Includes every state change
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

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
    let opt = Opt::from_args();

    let verbose = opt.verbose;

    let hyst = Time(opt.hyst);
    let maxidle = Time(opt.delay);

    let stdin = io::stdin();
    let mut file_reader;
    let mut stdin_reader;

    let input_read: &mut dyn BufRead = match opt.input {
        Some(filename) => {
            let file = File::open(filename);
            if file.is_err() {
                eprintln!("Could not open input file.");
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

    let mut trace_writer = match opt.output {
        Some(filename) => {
            let file = File::create(filename);
            if file.is_err() {
                eprintln!("Could not open trace file for writing.");
                ::std::process::exit(2);
            }
            BufWriter::new(Box::new(file.unwrap()) as Box<dyn Write>)
        }
        None => BufWriter::new(Box::new(stdout.lock()) as Box<dyn Write>),
    };

    let mut log_writer;
    match opt.log {
        Some(filename) => {
            let file = File::create(filename);
            if file.is_err() {
                eprintln!("Could not open log file for writing.");
                ::std::process::exit(2);
            }
            log_writer = Some(BufWriter::new(file.unwrap()));
        }
        None => log_writer = None,
    }

    let simul = simulator::Simulator::new(hyst, maxidle, PacketsFromRead::new(input_read));

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
