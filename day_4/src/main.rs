fn adjacency_req(candidate: &str) -> bool {
    candidate.as_bytes()
        .windows(2)
        .any(|pair| pair[0] == pair[1])
}

fn ascending_req(candidate: &str) -> bool {
    candidate.as_bytes()
        .windows(2)
        .all(|pair| pair[0] <= pair[1])
}

fn main() {
    let count = (372304..847061)
        .map(|x| format!("{}", x))
        .filter(|x| adjacency_req(&x))
        .filter(|x| ascending_req(&x))
        .count();

    println!("There were {} valid candidate passwords", count);
}