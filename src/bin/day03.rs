use aoc2020::read_to_string;

const TREE: &str = "#";

fn main() {
    let input = read_to_string("input/day03");
    // let mut trees = traverse_map(&input, 3, 1);

    let trees = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .copied()
        .fold(1usize, |mul, (r, d)| mul * traverse_map(&input, r, d));

    println!("encountered {} trees", trees);
}

fn traverse_map(map: &str, right: usize, down: usize) -> usize {
    let mut tree_counter = 0;
    for (i, line) in map.lines().step_by(down).enumerate() {
        let x_pos = i * right % line.len();
        if &line[x_pos..x_pos + 1] == TREE {
            tree_counter += 1;
        }
    }
    tree_counter
}
