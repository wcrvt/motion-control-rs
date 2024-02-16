
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