#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn main() {

    use digitalservo::algebra::dka_method::dka_method;
    let a = dka_method([1.0, -3.0, 2.0]);
    println!("{a:.04?}");

    // use digitalservo::algebra::*;
    // let m = Matrix::from([
    //     [0.0, 1.0, 3.0],
    //     [0.0, 0.0, 6.0],
    //     [7.0, 0.0, 0.0]
    // ]);

    // let lu = Eigen::crout_decomposition(m).unwrap();

    // const N: usize = 3;

    // println!("\nL:");
    // for i in 0..N { println!("{:.03?}", lu.l[i]); }
    // println!("\nU:");
    // for i in 0..N { println!("{:.03?}", lu.u[i]); }
    // println!("\nP:");
    // for i in 0..N { println!("{:.0?}", lu.p[i]); }

    // let n = lu.p.transpose() * lu.l * lu.u;
    // let r = (m - n).frobenius_norm();
    // println!("\nFrobenius norm of (M - Pt * L * U): {r:?}");
    
}
