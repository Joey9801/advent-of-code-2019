fn two_adjacent(candidate: &String) -> bool {
    candidate.as_bytes()
        .windows(4)
        .filter(|window| window[0] != window[1])
        .filter(|window| window[2] != window[3])
        .any(|window| window[1] == window[2])
}

fn ascending(candidate: &String) -> bool {
    candidate.as_bytes()
        .windows(2)
        .filter(|pair| pair[0] != b' ' && pair[1] != b' ')
        .all(|pair| pair[0] <= pair[1])
}

fn main() {
    let count = (372304..847061)
        .map(|x| format!(" {} ", x))
        .filter(two_adjacent)
        .filter(ascending)
        .count();

    println!("There were {} valid candidate passwords", count);
}