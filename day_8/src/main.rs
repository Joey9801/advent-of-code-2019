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

    let mut rendered = [' '; WIDTH * HEIGHT];
    for layer in layers.iter().rev() {
        for idx in 0..(WIDTH*HEIGHT) {
            match layer[idx] {
                0 => rendered[idx] = '░',
                1 => rendered[idx] = '█',
                2 => (),
                _ => unreachable!(),
            }
        }
    }

    for row in rendered.chunks(WIDTH) {
        for _repeat in 0..2 {
            for c in row {
                print!("{}{}{}", c, c, c);
            }
            print!("\n");
        }
    }
}