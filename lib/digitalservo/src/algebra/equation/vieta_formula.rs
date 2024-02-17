use num_complex::Complex;
use num_traits::Float;
use std::borrow::Borrow;

pub fn vieta_formula<T: Float, S: Borrow<[Complex<T>; N]>, const N: usize>(roots: S) -> [Complex<T>; N + 1]
where
    [(); N + 1]:,
{
    let roots: &[Complex<T>] = roots.borrow();
    let mut ret: [Complex<T>; N + 1] = [Complex {re: T::zero(), im: T::zero()}; N + 1];
    for k in 0..(N + 1) {
        ret[k] = get_ak(roots, k);
    }
    ret
}

fn get_ak<T: Float>(roots: &[Complex<T>], k: usize) -> Complex<T> {
    if k == 0 {
        return Complex {re: T::one(), im: T::zero()};
    };

    let sign: Complex<T> = if k % 2 == 0 {
        Complex {re: T::one(), im: T::zero()}
    } else {
        Complex {re: -T::one(), im: T::zero()}
    };

    let n: usize = roots.len();
    let mut combination = Combination::new(n);
    let combination: Vec<Vec<usize>> = combination.get_combination(k);

    let coefficient: Complex<T> = combination
        .iter()
        .map(|x| {
            x.iter()
            .fold(Complex {re: T::one(), im: T::zero()}, |a, &b| a * roots[b])
        })
        .fold(Complex { re: T::zero(), im: T::zero()}, |a, b| a + b) * sign;

    coefficient
}

struct Combination {
    combination: Vec<Vec<usize>>,
    n: usize,
    r: usize,
    cnt: usize,
}
impl Combination {
    pub fn new(n: usize) -> Self {
        let combination: Vec<Vec<usize>> = vec![];
        Self {
            combination,
            n,
            r: 0,
            cnt: 0,
        }
    }

    fn factorial_n_to_r(n: usize, r: usize) -> usize {
        if n == r { r } else { n * Self::factorial_n_to_r(n - 1, r) }
    }

    fn combination_n_r(n: usize, r: usize) -> usize {
        if r >= n || r <= 0 {
            1
        } else {
            let m: usize = if r < n - r { r } else { n - r };
            let r: usize = n - m + 1;
            Self::factorial_n_to_r(n, r) / Self::factorial_n_to_r(m, 1)
        }
    }

    fn search_index(&mut self, layer: usize, val: usize, combination: &Vec<usize>) {
        let mut combination: Vec<usize> = combination.clone();
        if val < self.n {
            if layer == 0 {
                for i in 0..self.n {
                    self.search_index(layer + 1, i, &combination);
                }
            } else if layer == self.r {
                combination[layer - 1] = val;
                self.combination[self.cnt] = combination;
                self.cnt += 1;
            } else {
                for i in (val + 1)..self.n {
                    combination[layer - 1] = val;
                    self.search_index(layer + 1, i, &combination);
                }
            }
        }
    }

    pub fn get_combination(&mut self, r: usize) -> Vec<Vec<usize>> {
        self.r = r;
        self.combination = vec![vec![0; r]; Self::combination_n_r(self.n, r)];
        self.search_index(0, 0, &vec![0; self.r]);
        self.combination.clone()
    }
}
