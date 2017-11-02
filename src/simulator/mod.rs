mod time;

use switch::{Switch, Packet, Status};
use std::iter::Iterator;
pub use self::time::Time;

pub struct Simulator<I: Iterator<Item = Packet>> {
    input: I,
    switch: Switch,
    current_time: Time,
    next_packet: Option<Packet>,
}

impl<I: Iterator<Item = Packet>> Iterator for Simulator<I> {
    type Item = (Time, Option<Packet>, Option<Status>);

    fn next(&mut self) -> Option<(Time, Option<Packet>, Option<Status>)> {
        match self.next_packet {
            Some(packet) => {
                let arrival_time = packet.arrival();
                let res = self.process();

                if self.current_time >= arrival_time {
                    self.next_packet = self.input.next();
                    if self.next_packet.is_some() {
                        self.switch.add_packet(&self.next_packet.unwrap());
                    }
                }

                Some(res)
            }
            None if self.switch.is_empty() => None,
            None => Some(self.process()),
        }
    }
}

impl<I: Iterator<Item = Packet>> Simulator<I> {
    pub fn new(hyst: Time, idle: Time, mut input: I) -> Simulator<I> {
        let packet = input.next();

        let mut s = Simulator {
            input: input,
            current_time: Time(0),
            switch: Switch::new(hyst, idle),
            next_packet: packet,
        };
        if s.next_packet.is_some() {
            s.switch.add_packet(&s.next_packet.unwrap());
        }

        s
    }

    fn process(&mut self) -> (Time, Option<Packet>, Option<Status>) {
        let res = self.switch.advance(self.current_time);

        self.current_time = res.time();

        let status = if res.state_change() {
            Some(self.switch.status())
        } else {
            None
        };

        (res.time(), res.packet(), status)
    }
}