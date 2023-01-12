use std::fs::read_to_string;

use proboscidea_volcanium::Graph;

fn main() {
    let graph = read_to_string("assets/example.txt")
        .expect("File")
        .parse::<Graph>()
        .expect("Parsed Graph");
    let solution = graph.get_solution(30);
    println!(
        "Score for 30 minutes: {}",
        graph.get_score(solution.iter(), 30)
    )
}
