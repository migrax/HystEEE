use simulator::Time;
use std::cmp;
use std::collections::VecDeque;
use std::fmt::{Display, Error, Formatter};

const T_S: Time = Time(2880);
const T_W: Time = Time(4480);
const CAPACITY: f64 = 10e9;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Packet {
    arrival: Time,
    size: u32,
}

impl Packet {
    pub fn new(arrival: Time, size: u32) -> Packet {
        Packet {
            arrival: arrival,
            size: size,
        }
    }

    pub fn arrival(&self) -> Time {
        self.arrival
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}


pub struct Switch {
    t_s: Time,
    t_w: Time,
    byte_time: f64,
    hyst: Time,
    idle: Time,
    status: Option<Box<SwitchStatus>>,
    queue: VecDeque<Packet>,
}

impl Switch {
    pub fn new(hyst: Time, idle: Time) -> Switch {
        Switch::new_explicit(hyst, idle, T_S, T_W, CAPACITY)
    }

    pub fn new_explicit(hyst: Time, idle: Time, ts: Time, tw: Time, capacity: f64) -> Switch {
        Switch {
            t_s: ts,
            t_w: tw,
            byte_time: 1e9 * 8.0 / capacity,
            hyst: hyst,
            idle: idle,
            status: Some(Box::new(Off::new(Time(0)))),
            queue: VecDeque::new(),
        }
    }

    fn tx_time(&self, packet: &Packet) -> Time {
        Time((f64::from(packet.size()) * self.byte_time).round() as u64)
    }

    pub fn add_packet(&mut self, packet: &Packet) {
        self.queue.push_back(*packet);
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn status(&self) -> Status {
        self.status.as_ref().unwrap().state()
    }

    pub fn advance(&mut self, now: Time) -> SwitchEvent {
        let res;

        if let Some(mut state) = self.status.take() {
            let ev = state.advance(now, self);
            res = SwitchEvent::new(&ev);
            self.status = Some(ev.status);
        } else {
            panic!("Switch is not in any state.");
        }

        res
    }
}

pub struct SwitchEvent {
    time: Time,
    packet: Option<Packet>,
    state_change: bool,
}

impl SwitchEvent {
    fn new(swev: &Event) -> SwitchEvent {
        SwitchEvent {
            time: swev.time,
            packet: swev.packet,
            state_change: swev.state_change,
        }
    }

    pub fn time(&self) -> Time {
        self.time
    }

    pub fn packet(&self) -> Option<Packet> {
        self.packet
    }
    pub fn state_change(&self) -> bool {
        self.state_change
    }
}

struct Event {
    status: Box<SwitchStatus>,
    time: Time,
    packet: Option<Packet>,
    state_change: bool,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Off,
    On,
    TOff,
    TOn,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match *self {
                Status::Off => "OFF",
                Status::On => "ON",
                Status::TOff => "T_OFF",
                Status::TOn => "T_ON",
            }
        )
    }
}

trait SwitchStatus {
    fn advance(&mut self, now: Time, switch: &mut Switch) -> Event;

    fn state(&self) -> Status;
}

struct Off {
    last_event: Time,
}

impl Off {
    fn new(last_event: Time) -> Off {
        Off {
            last_event: last_event,
        }
    }
}

impl SwitchStatus for Off {
    fn advance(&mut self, _now: Time, switch: &mut Switch) -> Event {
        let queue = &switch.queue;

        assert!(
            !queue.is_empty(),
            "Cannot run if Off state with empty queue"
        );

        let next_state = cmp::max(queue[0].arrival + switch.idle, self.last_event);

        self.last_event = next_state;

        Event {
            time: self.last_event,
            status: Box::new(TOn::new(next_state)),
            packet: None,
            state_change: true,
        }
    }

    fn state(&self) -> Status {
        Status::Off
    }
}

struct TOn {
    last_event: Time,
}

impl TOn {
    fn new(last_event: Time) -> TOn {
        TOn {
            last_event: last_event,
        }
    }
}

impl SwitchStatus for TOn {
    fn state(&self) -> Status {
        Status::TOn
    }

    fn advance(&mut self, _now: Time, switch: &mut Switch) -> Event {
        let queue = &switch.queue;

        assert!(
            !queue.is_empty(),
            "Cannot run if T_On state with empty queue"
        );

        let next_state = self.last_event + switch.t_w;
        self.last_event = next_state;

        Event {
            time: self.last_event,
            status: Box::new(On::new(next_state)),
            packet: None,
            state_change: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct On {
    last_event: Time,
    hyst_end: Time,
}

impl On {
    fn new(last_event: Time) -> On {
        On {
            last_event: last_event,
            hyst_end: last_event,
        }
    }
}

impl SwitchStatus for On {
    fn state(&self) -> Status {
        Status::On
    }

    fn advance(&mut self, now: Time, switch: &mut Switch) -> Event {
        let packet = {
            let queue = &mut switch.queue;

            assert!(!queue.is_empty());

            if queue[0].arrival() > now {
                let new_state: (Time, Box<SwitchStatus>) = if queue[0].arrival() > self.hyst_end {
                    (self.hyst_end, Box::new(TOff::new(self.hyst_end)))
                } else {
                    self.last_event = queue[0].arrival();
                    (self.last_event, Box::new(*self))
                };
                return Event {
                    time: new_state.0,
                    status: new_state.1,
                    packet: None,
                    state_change: true,
                };
            }

            queue.pop_front().unwrap()
        };

        let next_event = self.last_event + switch.tx_time(&packet);
        self.hyst_end = next_event + switch.hyst;

        self.last_event = next_event;

        Event {
            time: self.last_event,
            status: Box::new(*self),
            packet: Some(packet),
            state_change: false,
        }
    }
}

struct TOff {
    last_event: Time,
}

impl TOff {
    fn new(last_event: Time) -> TOff {
        TOff {
            last_event: last_event,
        }
    }
}

impl SwitchStatus for TOff {
    fn state(&self) -> Status {
        Status::TOff
    }

    fn advance(&mut self, _now: Time, switch: &mut Switch) -> Event {
        let next_state = self.last_event + switch.t_s;
        self.last_event = next_state;

        Event {
            time: self.last_event,
            status: Box::new(Off::new(next_state)),
            packet: None,
            state_change: true,
        }
    }
}
