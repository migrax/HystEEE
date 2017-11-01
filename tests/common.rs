#[cfg(test)]

extern crate eee_hyst;

use eee_hyst::simulator::{Simulator, Time};
use eee_hyst::switch::{Packet, Status};
use std::iter::Iterator;

pub fn setup<'a>(
    input: &'a [(u64, u32)],
    hyst: Time,
    idle: Time,
) -> Simulator<Box<Iterator<Item = Packet> + 'a>> {

    let input_iter = Box::new(input.into_iter().map(
        |entry| Packet::new(Time(entry.0), entry.1),
    ));

    Simulator::new(hyst, idle, input_iter)
}

fn adapt_sim<'a, I: Iterator<Item = (Time, Option<Packet>, Option<Status>)>>(
    input: &'a mut I,
) -> Box<'a + Iterator<Item = Packet>> {
    Box::new(
        input
            .filter(|ev| match *ev {
                (_, Some(_), _) => true,
                _ => false,
            })
            .map(|ev| Packet::new(ev.0, ev.1.unwrap().size())),
    )
}

pub fn compare<
    I: Iterator<Item = (Time, Option<Packet>, Option<Status>)>,
    E: Iterator<Item = Packet>,
>(
    mut res: &mut I,
    expected: E,
) -> Result<u32, (usize, Packet)> {
    for pair in adapt_sim(&mut res).zip(expected).enumerate() {
        match pair {
            (_, (packet_sim, packet_res)) if packet_sim == packet_res => continue,
            (i, (packet_sim, _)) => return Err((i + 1, packet_sim)),
        }
    }

    Ok(0)
}
