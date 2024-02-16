use super::*;

pub struct Combination <const N: usize, const R: usize>
where [(); combination(N, R)]:
{
    pub pair: [[usize; R]; combination(N, R)],
    pub cnt: usize,
}

impl <const N: usize, const R: usize> Combination<N, R>
where [(); combination(N, R)]:
{
    pub fn new() -> Self {
        Self {
            pair: [[0; R]; combination(N, R)],
            cnt : 0,
        }
    }

    pub fn get_pair(&mut self) {
        self.search_index(0, 0, [0; R]);
    }

    fn search_index(&mut self, layer: usize, val: usize, mut pair: [usize; R]) {
        if val < N {
            if layer == 0 {
                for i in 0..N {
                    self.search_index(layer + 1, i, pair);
                }
            }
            else if layer == R {
                pair[layer - 1] = val;
                self.pair[self.cnt] = pair;
                self.cnt += 1;
            } else {
                for i in (val + 1)..N {
                    pair[layer - 1] = val;
                    self.search_index(layer + 1, i, pair);
                }
            }
        }
    }
}


pub fn generate_combination_index<const N: usize, const R: usize> () -> [[usize; R]; combination(N, R)] {
    let mut combi: Combination<N, R> = Combination::<N, R>::new();
    combi.get_pair();
    combi.pair
}
