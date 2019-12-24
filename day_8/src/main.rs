const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Failed to read input");

    let levels = input
        .chars()
        .map(|c| c.to_digit(10).expect("Input character wasn't a digit"))
        .collect::<Vec<_>>();

    let layers = levels[..]
        .chunks(WIDTH * HEIGHT)
        .collect::<Vec<&[u32]>>();

    let min_zero_layer = layers
        .iter()
        .min_by_key(|l| l.iter().filter(|x| **x == 0).count())
        .expect("Failed to find the layer with the fewest zeros");

    let num_ones = min_zero_layer.iter().filter(|x| **x == 1).count();
    let num_twos = min_zero_layer.iter().filter(|x| **x == 2).count();
    let answer = num_ones * num_twos;
    dbg!(answer);
}