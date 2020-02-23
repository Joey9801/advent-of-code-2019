use std::io::{BufReader, Read};
use std::fs::File;
use std::path::Path;


trait Chop {
    fn chop(self) -> Self;
}

impl Chop for i32 {
    fn chop(self) -> i32 {
        self.abs() % 10
    }
}

struct PatternIterator {
    order: usize,
    n1: usize,
    n2: usize,
}

impl Iterator for PatternIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self.n2 % 4 {
            0 => 0,
            1 => 1,
            2 => 0,
            3 => -1,
            _ => panic!("usize % 4 returned outside the set {0, 1, 2, 3}"),
        };

        self.n1 += 1;
        if self.n1 == self.order {
            self.n1 = 0;
            self.n2 += 1;
        }

        Some(out)
    }
}

fn pattern(order: usize) -> impl Iterator<Item=i32> {
    PatternIterator {
        order,
        n1: 0,
        n2: 0,
    }
}

// Mutates the input signal with a single FFT round
fn fft_round(signal: &mut [i32]) {
    // A single round of fft is equivalent to multiplying an upper triangular matrix by the input
    // signal. Eg, for an input of length 5, [i1 .. i5], mapping to output [o1 .. o5]
    // [ o1 ]   [ 1  0 -1  0  1 ] [ i1 ]
    // [ o2 ]   [ 0  1  1  0  0 ] [ i2 ]
    // [ o3 ] = [ 0  0  1  1  1 ] [ i3 ]
    // [ o4 ]   [ 0  0  0  1  1 ] [ i4 ]
    // [ o5 ]   [ 0  0  0  0  1 ] [ i5 ]
    //
    // This means that oN is only influenced by iM, M>=N => the input vector can
    // be mutated in place without affecting the result iff the elements are
    // computed in order.

    for idx in 0..signal.len() {
        signal[idx] = pattern(idx + 1)
            .skip(1)
            .zip(signal.iter())
            .map(|(p, i)| p * i)
            .sum::<i32>()
            .chop();
    }
}

fn part_1(mut input: Vec<i32>) -> u64 {
    // Just perform the FFT rounds.
    // Input is only 650 long, so O(650^2 * 100) ~= O(4.2e7) operations

    for _ in 0..100 {
        fft_round(&mut input);
    }

    input[0..8].iter()
        .fold(0, |acc, num| acc * 10 + *num as u64)
}


// An infinite iterator of multipliers for part2
// n = 1 => all 1's
// n = 2 => ascending numbers (1, 2, 3, 4, ...)
// n = 3 => triangular numbers (1, 3, 6, 10, ...)
// etc..
// but all (mod 10), ie for n = 3, it actually outputs (1, 3, 6, 0, ...)
fn multiplier_sequence(n: i32) -> impl Iterator<Item=i32> {
    // Computes (a, b) (mod p) with Lucas's theorem
    // https://en.wikipedia.org/wiki/Lucas%27s_theorem
    fn lucas_binom(mut a: i32, mut b: i32, p: i32) -> i32 {
        // cache[a][b] == binom(a, b)
        let cache = [
            [1, 0, 0, 0, 0],
            [1, 1, 0, 0, 0],
            [1, 2, 1, 0, 0],
            [1, 3, 3, 1, 0],
            [1, 4, 6, 4, 1],
        ];

        let mut binom = 1;
        while b > 0 && binom > 0 {
            binom = binom * cache[(a % p) as usize][(b % p) as usize];
            a = a / p;
            b = b / p;
        }

        binom % p
    }

    (0..).map(move |i| {
        // Chinese remainder theorem to build x mod 10 from x mod 2 and x mod 5
        // Bezout identity for 5 and 2:
        //     1 * 5 + -2 * 2 = 1
        // => x mod 10 = 5 * (x mod 2) - 4 * (x mod 5)
        let mod_2 = lucas_binom(n + i - 1, i, 2);
        let mod_5 = lucas_binom(n + i - 1, i, 5);
        5 * mod_2 + -4 * mod_5
    })
}

fn part_2(input: Vec<i32>) -> u64 {
    // The matrix used in the FFT has the following properties:
    //  - is square
    //  - the Nth row (zero indexed) starts with N zeros, followed by N ones
    //      - matrix is upper triangular
    //      - The bottom ~1/2 of the rows are all [0, ..., 0, 1, ..., 1 ]
    //
    // The chop operation for non-negative numbers is just (mod 10), which is idempotent in
    // both addition and multiplication. Ie,
    //   ((a % 10) + (b % 10)) % 10 == (a + b) % 10
    //   ((a % 10) * (b % 10)) % 10 == (a * b) % 10
    //
    // Consider the reversed signal, S, and function returning the output of N
    // rounds of fft, f(S, N).
    // 
    // f(S, N)[0] = S[0].chop()
    // f(S, 1)[1] = (S[1] + S[0]).chop()
    // f(S, 1)[2] = (S[0] + S[1] + S[2]).chop()
    //
    // f(S, 1)[M] = S[M]
    // f(S, N)[M] = \sum{i=0}{M}{ f(S, N-1)[i] k}.chop()
    //            = \sum{i=0}{len(S) - M}{ binom(N + i - 1, i) * S[i + M] }.chop()
    //
    // binom(N + i - 1, i) will probably overflow for large N + i, so use Lucas's
    // theorem + the chinese remainder theorem to compute it mod 10. That
    // computation is in the `multiplier_sequence(N)` method.

    let offset = input[0..7]
        .iter()
        .fold(0, |acc, num| acc * 10 + *num as usize);
    let signal_len = input.len() * 10_000;
    assert!(offset as f32 / signal_len as f32 > 0.5);


    // Access elements of the repeated signal, avoiding allocating a large buffer for it
    let access = |idx: usize| {
        input[idx % input.len()]
    };

    // Value after 100 iterations of the reversed index
    let final_value_at = |idx: usize| -> i32 {
        (idx..(input.len() * 10_000))
            .zip(multiplier_sequence(100))
            .map(|(i, mul)| access(i) * mul)
            .sum::<i32>() % 10
    };

    (offset..(offset + 8))
        .map(final_value_at)
        .fold(0, |acc, num: i32| acc * 10 + num as u64)
}

fn main() {
    let file = File::open(Path::new("./input.txt"))
        .expect("Failed to open puzzle input");
    let mut reader = BufReader::new(file);

    let mut input_string = String::new();
    reader.read_to_string(&mut input_string)
        .expect("Failed to read file contents");

    let input = input_string.chars()
        .map(|c| c.to_digit(10).expect("Input byte wasn't an ascii number"))
        .map(|num| num as i32)
        .collect::<Vec<_>>();

    dbg!(part_1(input.clone()));
    dbg!(part_2(input.clone()));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chop() {
        assert_eq!(0.chop(), 0);
        assert_eq!(1.chop(), 1);
        assert_eq!(9.chop(), 9);

        assert_eq!(10.chop(), 0);
        assert_eq!(11.chop(), 1);
        assert_eq!(19.chop(), 9);

        assert_eq!((-10).chop(), 0);
        assert_eq!((-11).chop(), 1);
        assert_eq!((-19).chop(), 9);
    }

    #[test]
    fn test_pattern() {
        assert_eq!(pattern(1).take(8).collect::<Vec<_>>(), vec![0, 1, 0, -1, 0, 1, 0, -1]);
        assert_eq!(pattern(2).take(8).collect::<Vec<_>>(), vec![0, 0, 1, 1, 0, 0, -1, -1]);
    }

    #[test]
    fn test_fft_round() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
        fft_round(&mut nums);
        assert_eq!(nums, vec![4, 8, 2, 2, 6, 1, 5, 8]);
    }

    #[test]
    fn test_multiplier_sequence() {
        let seq_1: Vec<_> = multiplier_sequence(1).take(5).collect();
        assert_eq!(seq_1, vec![1, 1, 1, 1, 1]);

        let seq_2: Vec<_> = multiplier_sequence(2).take(5).collect();
        assert_eq!(seq_2, vec![1, 2, 3, 4, 5]);

        let seq_3: Vec<_> = multiplier_sequence(3).take(5).collect();
        // 1, 3, 6, 10, 15 (mod 10)
        assert_eq!(seq_3, vec![1, 3, 6, 0, 5]);

        let seq_4: Vec<_> = multiplier_sequence(4).take(5).collect();
        // 1, 4, 10, 20, 35 (mod 10)
        assert_eq!(seq_4, vec![1, 4, 0, 0, 5]);
    }
}
