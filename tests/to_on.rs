#[cfg(test)]

extern crate eee_hyst;
mod common;

use eee_hyst::simulator::Time;
use eee_hyst::switch::Packet;
use common::*;

#[test]
fn from_off() {
    let input = vec![(100, 1000)];
    let expected = vec![Packet::new(Time(5380), 1000)];
    let mut sim = setup(&input, Time(0), Time(0));

    if let Err((i, packet)) = compare(&mut sim, expected.into_iter()) {
        panic!("{}th packets yielded {:?}", i, packet);
    }
}


#[test]
fn from_off_delay() {
    let input = vec![(100, 1000)];
    let expected = vec![Packet::new(Time(5880), 1000)];
    let mut sim = setup(&input, Time(0), Time(500));

    if let Err((i, packet)) = compare(&mut sim, expected.into_iter()) {
        panic!("{}th packets yielded {:?}", i, packet);
    }
}
