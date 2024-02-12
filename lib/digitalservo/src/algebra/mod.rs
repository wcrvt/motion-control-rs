pub mod matrix;
pub mod vector;


#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    pub data: [[T; COLS]; ROWS],
}

#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const ROWS: usize> {
    pub data: [T; ROWS],
}