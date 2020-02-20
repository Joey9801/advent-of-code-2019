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

fn main() {
    let file = File::open(Path::new("./input.txt"))
        .expect("Failed to open puzzle input");
    let mut reader = BufReader::new(file);

    let mut nums_string = String::new();
    reader.read_to_string(&mut nums_string)
        .expect("Failed to read file contents");

    let mut nums = nums_string.chars()
        .map(|c| c.to_digit(10).expect("Input byte wasn't an ascii number"))
        .map(|num| num as i32)
        .collect::<Vec<_>>();

    for _ in 0..100 {
        fft_round(&mut nums);
    }

    dbg!(&nums[0..8]);
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
}