
pub mod permutation;
pub mod combination;

pub use permutation::*;
pub use combination::*;

pub const fn factorial(n: usize) -> usize {
    if n == 0 { 1 } else { n * factorial(n - 1)}
}

pub const fn factorial_n_to_r(n: usize, r: usize) -> usize {
    if n == r { r } else { n * factorial_n_to_r(n - 1, r)}
}

pub const fn permutation(n: usize, r: usize) -> usize {
    let r: usize = n - r + 1;
    factorial_n_to_r(n, r)
}

pub const fn combination(n: usize, r: usize) -> usize {
    let m: usize = if r < n - r { r } else { n - r };
    let r: usize = n - m + 1;
    factorial_n_to_r(n, r) / factorial(m)
}