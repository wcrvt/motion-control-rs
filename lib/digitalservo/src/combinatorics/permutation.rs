use super::*;

fn swap<T: Copy, const N: usize>(mut v: [T; N], a: usize, b: usize) -> [T; N] {
    let buffer: T = v[a];
    v[a] = v[b];
    v[b] = buffer;
    v
}

pub fn generate_permutation_index<const N: usize> () -> [[usize; N]; permutation(N, N)]
where
    [(); permutation(N, N)]:
{
    let mut base: [usize; N] = [0; N];
    for i in 0..N { base[i] = i as usize; }

    let mut cnt: usize = 0;
    let mut ret: [[usize; N]; permutation(N, N)] = [[0; N]; permutation(N, N)];
    ret[cnt] = base;
    cnt += 1;

    for i in 0..N {
        for j in 0..cnt {
            for k in (i + 1)..N {
                ret[cnt] = swap(ret[j], i, k);
                cnt += 1;
            }
        }
    }
    ret
}
