#[cfg(test)]

extern crate eee_hyst;
mod common;

use eee_hyst::simulator::Time;
use eee_hyst::switch::Packet;
use crate::common::*;

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
    let input = vec![(100, 1000), (9000, 1001)];
    let expected = vec![
        Packet::new(Time(5880), 1000),
        Packet::new(Time(14_781), 1001),
    ];

    let mut sim = setup(&input, Time(0), Time(500));

    if let Err((i, packet)) = compare(&mut sim, expected.into_iter()) {
        panic!("{}th packets yielded {:?}", i, packet);
    }
}

#[test]
fn to_off_and_back_delay_delay_gt_ts() {
    let input = vec![(100, 1000), (13010, 1001), (30000, 1002)];
    let expected = vec![
        Packet::new(Time(10_380), 1000),
        Packet::new(Time(23_291), 1001),
        Packet::new(Time(40_282), 1002),
    ];

    let mut sim = setup(&input, Time(0), Time(5000));

    if let Err((i, packet)) = compare(&mut sim, expected.into_iter()) {
        panic!("{}th packets yielded {:?}", i, packet);
    }
}


#[test]
fn to_off_and_back_delay_no_toff() {
    let input = vec![(100, 1000), (6000, 1001)];
    let expected = vec![
        Packet::new(Time(5880), 1000),
        Packet::new(Time(14_041), 1001),
    ];

    let mut sim = setup(&input, Time(0), Time(500));

    if let Err((i, packet)) = compare(&mut sim, expected.into_iter()) {
        panic!("{}th packets yielded {:?}", i, packet);
    }
}

#[test]
fn to_off_and_back_delay_no_toff_250_in_off() {
    let input = vec![(100, 1000), (8510, 1001)];
    let expected = vec![
        Packet::new(Time(5880), 1000),
        Packet::new(Time(14_291), 1001),
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
