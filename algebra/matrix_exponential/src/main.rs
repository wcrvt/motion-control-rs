use digitalservo::algebra::*;

pub const P: usize = 1000;

fn main() {
    let m = Matrix::from([[2.0, 2.0], [2.0, 5.0]]);

    let mut eigen = Eigen::qr_method(&m);
    println!("{:.04?}", eigen.value);
    println!("{:.04?}", eigen.vector);

    let q = eigen.get_matrix_exponential().unwrap();
    println!("{:.04?}", q);

    let q = m.exp();
    println!("{:.04?}", q.data);

    let v = Vector::from([1,2,3]);
    let mut w = Vector::from([1,2,3]);
    w += v;
    println!("{:?}", w);
}
