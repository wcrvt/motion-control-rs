use digitalservo::algebra::*;

fn main() {
    let m = Matrix::from([[5.0, 1.0], [-100.0, -20.0]]);

    let eigen = Eigen::qr_method(m);
    println!("{:.04?}", eigen.value);
    println!("{:.04?}", eigen.vector);

    println!("{:.04?}", m.determinant());
}
