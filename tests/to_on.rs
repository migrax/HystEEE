#[cfg(test)]

extern crate eee_hyst;
mod common;

use eee_hyst::simulator::Time;
use eee_hyst::switch::Packet;
use common::*;

#[test]
fn from_off() {
    let input = vec![(100, 1000), (5000, 5000)];
    let expected = vec![Packet::new(Time(5380), 1000)];
    let mut sim = setup(&input, Time(0), Time(0));

    match compare(&mut sim, expected.into_iter()) {
        Err((i, packet)) => panic!("{}th packets yielded {:?}", i, packet),
        Ok(_) => {}
    }
}


#[test]
fn from_off_delay() {
    let input = vec![(100, 1000), (5000, 5000)];
    let expected = vec![Packet::new(Time(5880), 1000)];
    let mut sim = setup(&input, Time(0), Time(500));

    match compare(&mut sim, expected.into_iter()) {
        Err((i, packet)) => panic!("{}th packets yielded {:?}", i, packet),
        Ok(_) => {}
    }
}
