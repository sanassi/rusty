mod matrix;

fn main() {
    let m = &matrix::Matrix::new(1, 2, 2);
    let n = &matrix::Matrix::new(3, 2, 2);
    let sum = m + n;

    sum.print();
}
