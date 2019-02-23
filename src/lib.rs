use std::{ops, fmt};

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        Matrix{ data: values.to_vec(), row: row, col: col }
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        Matrix{ data: Vec::new(), row: row, col: col }
    }

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        }
        else {
            let mut add_matrix = Matrix::new_empty(self.row, self.col);
            for i in 0..self.data.len() {
                add_matrix.data.push(self.data[i] + rhs.data[i]);
            }
            add_matrix
        }
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        } else {
            let mut add_matrix = Matrix::new_empty(self.row, self.col);
            for i in 0..self.data.len() {
                add_matrix.data.push(self.data[i] + rhs.data[i]);
            }
            add_matrix
        }
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        } else {
            let mut add_matrix = Matrix::new_empty(self.row, self.col);
            for i in 0..self.data.len() {
                add_matrix.data.push(self.data[i] + rhs.data[i]);
            }
            add_matrix
        }
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: &Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        } else {
            let mut add_matrix = Matrix::new_empty(self.row, self.col);
            for i in 0..self.data.len() {
                add_matrix.data.push(self.data[i] + rhs.data[i]);
            }
            add_matrix
        }
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        } else {
            let mut sub_matrix = Matrix::new_empty(self.row, self.col);
            for i in 0..self.data.len() {
                sub_matrix.data.push(self.data[i] - rhs.data[i]);
            }
            sub_matrix
        }
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        } else {
            let mut sub_matrix = Matrix::new_empty(self.row, self.col);
            for i in 0..self.data.len() {
                sub_matrix.data.push(self.data[i] - rhs.data[i]);
            }
            sub_matrix
        }
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        } else {
            let mut sub_matrix = Matrix::new_empty(self.row, self.col);
            for i in 0..self.data.len() {
                sub_matrix.data.push(self.data[i] - rhs.data[i]);
            }
            sub_matrix
        }
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: &Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        } else {
            let mut sub_matrix = Matrix::new_empty(self.row, self.col);
            for i in 0..self.data.len() {
                sub_matrix.data.push(self.data[i] - rhs.data[i]);
            }
            sub_matrix
        }
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        if self.col != rhs.row {
            panic!();
        } else {
            let mut mul_matrix = Matrix::new_empty(self.row, rhs.col);
            for first_row_index in (0..self.data.len()).step_by(self.col) {
                for second_col_index in 0..rhs.col {
                    let mut store_vec: Vec<T> = Vec::new();
                    let mut second_ele_index = second_col_index;
                    for first_ele_index in (first_row_index..).take(self.col) {
                        store_vec.push(self.data[first_ele_index] * rhs.data[second_ele_index]);
                        second_ele_index += rhs.col;
                    }
                    let mut sum = store_vec[0];
                    for index in 1..store_vec.len() {
                        sum = sum + store_vec[index];
                    }
                    mul_matrix.data.push(sum);
                }
            }
            mul_matrix
        }
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.col != rhs.row {
            panic!();
        } else {
            let mut mul_matrix = Matrix::new_empty(self.row, rhs.col);
            for first_row_index in (0..self.data.len()).step_by(self.col) {
                for second_col_index in 0..rhs.col {
                    let mut store_vec: Vec<T> = Vec::new();
                    let mut second_ele_index = second_col_index;
                    for first_ele_index in (first_row_index..).take(self.col) {
                        store_vec.push(self.data[first_ele_index] * rhs.data[second_ele_index]);
                        second_ele_index += rhs.col;
                    }
                    let mut sum = store_vec[0];
                    for index in 1..store_vec.len() {
                        sum = sum + store_vec[index];
                    }
                    mul_matrix.data.push(sum);
                }
            }
            mul_matrix
        }
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        if self.col != rhs.row {
            panic!();
        } else {
            let mut mul_matrix = Matrix::new_empty(self.row, rhs.col);
            for first_row_index in (0..self.data.len()).step_by(self.col) {
                for second_col_index in 0..rhs.col {
                    let mut store_vec: Vec<T> = Vec::new();
                    let mut second_ele_index = second_col_index;
                    for first_ele_index in (first_row_index..).take(self.col) {
                        store_vec.push(self.data[first_ele_index] * rhs.data[second_ele_index]);
                        second_ele_index += rhs.col;
                    }
                    let mut sum = store_vec[0];
                    for index in 1..store_vec.len() {
                        sum = sum + store_vec[index];
                    }
                    mul_matrix.data.push(sum);
                }
            }
            mul_matrix
        }
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: &Self) -> Self::Output {
        if self.col != rhs.row {
            panic!();
        } else {
            let mut mul_matrix = Matrix::new_empty(self.row, rhs.col);
            for first_row_index in (0..self.data.len()).step_by(self.col) {
                for second_col_index in 0..rhs.col {
                    let mut store_vec: Vec<T> = Vec::new();
                    let mut second_ele_index = second_col_index;
                    for first_ele_index in (first_row_index..).take(self.col) {
                        store_vec.push(self.data[first_ele_index] * rhs.data[second_ele_index]);
                        second_ele_index += rhs.col;
                    }
                    let mut sum = store_vec[0];
                    for index in 1..store_vec.len() {
                        sum = sum + store_vec[index];
                    }
                    mul_matrix.data.push(sum);
                }
            }
            mul_matrix
        }
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut outstr = "".to_owned();
        for index in 0..self.data.len() {
            if index % &self.col == &self.col - 1 {
                outstr = outstr + &format!("{}", self.data[index]) + "\n";
            } else {
                outstr = outstr + &format!("{}", self.data[index]) + " ";
            }
        }
        write!(f, "{}", outstr)
    }
}

fn main() {
    //test();
}

fn test() {
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(3, 2, &[1, 2, 3, 4, 5, 6]);
//    assert_eq!(&x + &y - &y, x);
    let z = &x * &y;
//    println!("{:?}", z);
    assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
    println!("{:?}\n{:?}", format!("{}", x), format!("{}", z));
}