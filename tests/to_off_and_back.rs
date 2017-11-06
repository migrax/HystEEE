#[cfg(test)]

extern crate eee_hyst;
mod common;

use eee_hyst::simulator::Time;
use eee_hyst::switch::Packet;
use common::*;

#[test]
fn to_off_and_back() {
    let input = vec![(100, 1000), (6000, 1001)];
    let expected = vec![
        Packet::new(Time(5380), 1000),
        Packet::new(Time(13_541), 1001),
    ];

    let mut sim = setup(&input, Time(0), Time(0));

    if let Err((i, packet)) = compare(&mut sim, expected.into_iter()) {
        panic!("{}th packets yielded {:?}", i, packet);
    }
}

#[test]
fn to_off_and_back_delay() {
    let input = vec![(100, 1000), (6000, 1001)];
    let expected = vec![
        Packet::new(Time(5880), 1000),
        Packet::new(Time(14_541), 1001),
    ];

    let mut sim = setup(&input, Time(0), Time(500));

    if let Err((i, packet)) = compare(&mut sim, expected.into_iter()) {
        panic!("{}th packets yielded {:?}", i, packet);
    }
}

#[test]
fn to_off_and_back_hyst() {
    let input = vec![(100, 1000), (6000, 1001)];
    let expected = vec![
        Packet::new(Time(5380), 1000),
        Packet::new(Time(14_041), 1001),
    ];

    let mut sim = setup(&input, Time(500), Time(0));

    if let Err((i, packet)) = compare(&mut sim, expected.into_iter()) {
        panic!("{}th packets yielded {:?}", i, packet);
    }
}
