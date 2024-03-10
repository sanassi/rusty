use std::ops;

pub struct Matrix {
    pub w: usize,
    pub h: usize,
    pub data: Vec<i32>
}

impl Matrix {
    pub fn new(init: i32, w: usize, h: usize) -> Matrix {
        Matrix {
            w: w,
            h: h,
            data: vec![init; w * h]
        }
    }

    pub fn print(&self) {
        print!("[");
        for elt in self.data.iter() {
            print!(" {elt}");
        }

        println!(" ]");
    }
}

impl ops::Add<&Matrix> for &Matrix {
    type Output = Matrix;
    fn add(self, _rhs: &Matrix) -> Matrix {
        assert_eq!(self.w, _rhs.w);
        assert_eq!(self.h, _rhs.h);

        let mut res = Matrix::new(0, _rhs.w, _rhs.h);
        for (pos, e) in self.data.iter().enumerate() {
            res.data[pos] = e + _rhs.data[pos];
        }
        res
    }
}
