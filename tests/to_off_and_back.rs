#[cfg(test)]

extern crate eee_hyst;
mod common;

use eee_hyst::simulator::Time;
use eee_hyst::switch::Packet;
use common::*;

#[test]
fn to_off_and_back() {
    let input = vec![(100, 1000), (6000, 1001), (100000, 100000)];
    let expected = vec![
        Packet::new(Time(5380), 1000),
        Packet::new(Time(13541), 1001),
    ];

    let mut sim = setup(&input, Time(0), Time(0));

    match compare(&mut sim, expected.into_iter()) {
        Err((i, packet)) => panic!("{}th packets yielded {:?}", i, packet),
        Ok(_) => {}
    }
}

#[test]
fn to_off_and_back_delay() {
    let input = vec![(100, 1000), (6000, 1001), (100000, 100000)];
    let expected = vec![
        Packet::new(Time(5880), 1000),
        Packet::new(Time(14541), 1001),
    ];

    let mut sim = setup(&input, Time(0), Time(500));

    match compare(&mut sim, expected.into_iter()) {
        Err((i, packet)) => panic!("{}th packets yielded {:?}", i, packet),
        Ok(_) => {}
    }
}

#[test]
fn to_off_and_back_hyst() {
    let input = vec![(100, 1000), (6000, 1001), (100000, 100000)];
    let expected = vec![
        Packet::new(Time(5380), 1000),
        Packet::new(Time(14041), 1001),
    ];

    let mut sim = setup(&input, Time(500), Time(0));

    match compare(&mut sim, expected.into_iter()) {
        Err((i, packet)) => panic!("{}th packets yielded {:?}", i, packet),
        Ok(_) => {}
    }
}
