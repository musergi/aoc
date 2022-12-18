use std::fs;

use distress_signal::Packet;

fn main() {
    let mut packets = fs::read_to_string("assets/input.txt")
        .expect("Read file")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Packet>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Parsed packets");
    let pairs = packets
        .iter()
        .cloned()
        .fold(Vec::<Vec<Packet>>::new(), |mut acc, packet| {
            match acc.pop() {
                Some(mut last) => {
                    if last.len() == 2 {
                        acc.push(last);
                        acc.push(vec![packet]);
                    } else {
                        last.push(packet);
                        acc.push(last);
                    }
                }
                None => acc.push(vec![packet]),
            }
            acc
        });
    let ordered = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if p.get(0).unwrap() <= p.get(1).unwrap() {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("Sum: {}", ordered);
    packets.push(Packet::first());
    packets.push(Packet::second());
    packets.sort();
    let first = Packet::first();
    let second = Packet::second();
    let first = packets
        .iter()
        .enumerate()
        .find_map(|(i, p)| if *p == first { Some(i + 1) } else { None })
        .expect("First flag");
    let second = packets
        .iter()
        .enumerate()
        .find_map(|(i, p)| if *p == second { Some(i + 1) } else { None })
        .expect("Second flag");
    println!("Signal: {}", first * second);
}
