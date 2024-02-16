use digitalservo::algebra::*;

fn main() {

    let m = Matrix::from([
        [0.0, 2.0, 3.0],
        [4.0, 0.0, 6.0],
        [7.0, 8.0, 9.0]
    ]);

    let lu = Eigen::crout_decomposition(m).unwrap();

    const N: usize = 3;

    println!("\nL:");
    for i in 0..N { println!("{:.03?}", lu.l[i]); }
    println!("\nU:");
    for i in 0..N { println!("{:.03?}", lu.u[i]); }
    println!("\nP:");
    for i in 0..N { println!("{:.03?}", lu.p[i]); }

    let n = lu.p.transpose() * lu.l * lu.u;
    println!("\nM:");
    for i in 0..N { println!("{:.03?}", m[i]); }
    println!("\nN:");
    for i in 0..N { println!("{:.03?}", n[i]); }
    
}
