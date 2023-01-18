use std::fs;

use proboscidea_volcanium::Volcano;

fn main() {
    let infos = fs::read_to_string("assets/input.txt")
        .expect("Read file")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();
    let volcano = Volcano::from(infos);
    println!("{}", volcano.get_max_flow(30));
}
