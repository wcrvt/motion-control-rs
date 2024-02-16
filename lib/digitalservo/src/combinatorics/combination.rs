use super::*;

pub const fn combination(n: usize, r: usize) -> usize {
    let m: usize = if r < n - r { r } else { n - r };
    let r: usize = n - m + 1;
    factorial_n_to_r(n, r) / factorial(m)
}

pub struct Combination <const N: usize, const R: usize>
where [(); combination(N, R)]:
{
    pub combination: [[usize; R]; combination(N, R)],
    pub cnt: usize,
}

impl <const N: usize, const R: usize> Combination<N, R>
where [(); combination(N, R)]:
{
    pub fn new() -> Self {
        Self {
            combination: [[0; R]; combination(N, R)],
            cnt : 0,
        }
    }

    pub fn get_combination(&mut self) {
        self.search_index(0, 0, [0; R]);
    }

    fn search_index(&mut self, layer: usize, val: usize, mut combination: [usize; R]) {
        if val < N {
            if layer == 0 {
                for i in 0..N {
                    self.search_index(layer + 1, i, combination);
                }
            }
            else if layer == R {
                combination[layer - 1] = val;
                self.combination[self.cnt] = combination;
                self.cnt += 1;
            } else {
                for i in (val + 1)..N {
                    combination[layer - 1] = val;
                    self.search_index(layer + 1, i, combination);
                }
            }
        }
    }
}


pub fn generate_combination_index<const N: usize, const R: usize> () -> [[usize; R]; combination(N, R)] {
    let mut combi: Combination<N, R> = Combination::<N, R>::new();
    combi.get_combination();
    combi.combination
}
