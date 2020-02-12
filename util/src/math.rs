use crate::integer::Integer;

pub fn gcd<T: Integer>(a: T, b: T) -> T {
    if b == T::zero() { a } else { gcd(b, a % b) }
}

pub fn lcm<T: Integer>(a: T, b: T) -> T {
    a * b  / gcd(a, b)
}

pub fn lcm3<T: Integer>(a: T, b: T, c: T) -> T {
    lcm(a, lcm(b, c))
}