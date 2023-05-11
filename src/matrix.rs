use std::ops;
use std::cmp;

/// Represents a mathematical matrix, zero-indexed
pub struct Matrix {
    matrix : Vec<Vec<f64>>,
    rows : usize,
    columns : usize
}


impl Matrix {
    /// Creates a new zero matrix with the given size parameters
    pub fn new(rows : usize, columns : usize) -> Self{
        let mut matrix : Vec<Vec<f64>> = Vec::with_capacity(rows);

        for row_index in 0..rows{
            matrix.push(Vec::with_capacity(columns));
            for _column_index in 0..columns{
                matrix[row_index].push(0.0);
            }
        }

        Self {
            matrix,
            rows,
            columns
        }
    }

    /// Creates a new square zero matrix with the given size parameters
    pub fn square_matrix(size : usize) -> Self{
        return Self::new(size, size);
    }

    /// Creates a new matrix from the given 2D vector array. The array must have consistent rectangular sizing
    pub fn from_vector(vector : &Vec<Vec<f64>>) -> Self{
        let rows : usize = vector.capacity();
        let columns : usize = vector[0].capacity();

        for row in vector {
            if columns != row.capacity() {
                panic!("This matrix doesn't have equal column sizes!")
            }
        }

        let mut matrix : Vec<Vec<f64>> = Vec::with_capacity(rows);

        for row_index in 0..rows {
            matrix.push(vector[row_index].clone());
        }

        Self {
            matrix,
            rows,
            columns
        }
    }

    /// Creates a new identity matrix with the given size
    pub fn identity_matrix(size : usize) -> Self{
        let mut matrix : Matrix = Self::square_matrix(size);
        
        for i in 0..matrix.rows {
            matrix.set_value(i, i, 1.0);
        }

        return matrix;
    }

    /// Gets the value of the matrix at the given indices (0 indexed). Functionally equivalent to Matrix\[row\]\[column\]
    pub fn get_value(&self, row : usize, column : usize) -> f64{
        return self.matrix[row][column];
    }

    /// Sets the value of the matrix at the given indices (0 indexed)
    pub fn set_value(&mut self, row : usize, column : usize, value : f64) {
        self.matrix[row][column] = value;
    }

    /// Calculates the inner product of two input Vec<f64> objects
    fn inner_product(a : Vec<f64>, b : Vec<f64>) -> f64{
        if a.len() != b.len() {
            panic!("These vectors are of different sizes!");
        }

        let mut output : f64 = 0.0;

        for i in 0..a.len() {
            output += a[i] * b[i];
        }

        return output;
    }

    /// Calculates the reduced echelon form of this matrix, and also returns the determinant (0 if the matrix is non-square)
    pub fn reduced_echelon_and_det(&self, determinant : &mut f64) -> Matrix {
        let mut operating_matrix : Vec<Vec<f64>> = self.clone().matrix;

        let mut current_pivot_row : usize = 0;
        let mut current_pivot_column : usize = 0;
        let mut factor : f64;
        *determinant = 1.0;

        while self.rows - current_pivot_row > 0 && self.columns - current_pivot_column > 0 {
            let mut changed : bool = false;

            // Find the next pivot
            for column in current_pivot_column..self.columns {
                for row in current_pivot_row..self.rows {
                    if operating_matrix[row][column] != 0.0 {
                        // Row swap if necessary
                        if current_pivot_row != row {
                            let temp : Vec<f64> = operating_matrix[row].clone();
                            operating_matrix[row] = operating_matrix[current_pivot_row].clone();
                            operating_matrix[current_pivot_row] = temp;
                            *determinant = -1.0 * *determinant;
                        }
                        // Update the column
                        current_pivot_column = column;
                        changed = true;
                        break;
                    }
                }
                if changed {
                    break;
                }
            }

            // If the pivot didn't change, then we have exhausted all pivots
            if !changed {
                break;
            }

            // Set the pivot to 1
            factor = operating_matrix[current_pivot_row][current_pivot_column];
            for column in current_pivot_column..self.columns {
                operating_matrix[current_pivot_row][column] /= factor;
            }
            *determinant *= factor;

            // Reduce down all rows above and underneath
            for row in 0..self.rows {
                if operating_matrix[row][current_pivot_column] == 0.0 || row == current_pivot_row {
                    continue;
                }
                factor = operating_matrix[row][current_pivot_column];
                for column in current_pivot_column..self.columns {
                    operating_matrix[row][column] -= factor * operating_matrix[current_pivot_row][column];
                }
            }

            // Force the pivot to update
            current_pivot_row += 1;
            current_pivot_column += 1;
        }

        // Checks if this matrix is square and has so has a determinant, then checks that this matrix is equal to In 
        if self.rows != self.columns {
            *determinant = 0.0;
        }
        else {
            for i in 0..self.rows {
                if operating_matrix[i][i] == 0.0 {
                    *determinant = 0.0;
                    break;
                }
            }
        }

        return Self::from_vector(&operating_matrix);
    }

    /// Calculates and returns the reduced echelon form of this matrix
    pub fn reduced_echelon_form(&self) -> Matrix {
        let determinant: &mut f64 = &mut 0.0;
        return self.reduced_echelon_and_det(determinant)
    }

    /// Calculates and returns the determinant if this matrix is square
    pub fn determinant(&self) -> f64 {
        if self.rows != self.columns {
            panic!("This matrix is not square!");
        }
        let determinant: &mut f64 = &mut 0.0;
        self.reduced_echelon_and_det(determinant);
        return *determinant;
    }

    /// Calculates and returns the inverse of this matrix, if this matrix is invertible
    pub fn inverse(&self) -> Result<Matrix, &'static str> {
        if self.rows != self.columns {
            panic!("This matrix is not square!");
        }

        let identity_matrix : Matrix = Matrix::identity_matrix(self.rows);
        let mut reduced_echelon_form_vector : Vec<Vec<f64>> = Vec::with_capacity(self.rows * 2);

        for row in 0..self.rows {
            reduced_echelon_form_vector.push(self[row].clone());
        }
        for row in 0..self.rows {
            reduced_echelon_form_vector[row].append(&mut (identity_matrix[row].clone()));
        }

        let reduced_matrix : Matrix = Matrix::from_vector(&reduced_echelon_form_vector).reduced_echelon_form();

        for row in 0..self.rows {
            for column in 0..self.columns {
                if reduced_matrix[row][column] != identity_matrix[row][column] {
                    return Err("Matrix is not invertible");
                }
            }
        }

        let mut inverse_matrix : Vec<Vec<f64>> = Vec::with_capacity(self.rows);

        for row in 0..self.rows {
            inverse_matrix.push(reduced_matrix[row + self.rows].clone());
        }

        return Ok(Matrix::from_vector(&inverse_matrix));
    }

    /// Returns a transpose of this matrix
    pub fn transpose(&self) -> Matrix {
        let mut transpose_matrix : Matrix = Matrix::new(self.columns, self.rows);

        for row in 0..self.rows {
            for column in 0..self.columns {
                transpose_matrix.set_value(column, row, self[row][column]);
            }
        }

        return transpose_matrix;
    }
}

impl Clone for Matrix {
    /// Safely clones this matrix
    fn clone(&self) -> Self {
        let mut matrix : Vec<Vec<f64>> = Vec::with_capacity(self.rows);

        for i in 0..self.rows {
            matrix.push(self.matrix[i].clone());
        }

        return Matrix {
            matrix: matrix,
            rows: self.rows,
            columns: self.columns
        }
    }
}

impl ops::Add for Matrix {
    type Output = Matrix;

    /// Adds two matrices together
    fn add(self, rhs: Matrix) -> Matrix {
        if self.rows != rhs.rows || self.columns != rhs.columns {
            panic!("Matrix size mismatch!");
        }

        let mut output : Matrix = Matrix::new(self.rows, self.columns);

        for row_index in 0..self.rows {
            for column_index in 0..self.columns {
                let value : f64 = self[row_index][column_index] + rhs[row_index][column_index];
                output.set_value(row_index, column_index, value);
            }
        }

        return output;
    }
}

impl ops::AddAssign for Matrix {
    /// Adds and reassigns two matrices together
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl ops::Sub for Matrix {
    type Output = Matrix;

    /// Subtracts the two matrices. Equivalent to self + rhs * -1.0
    fn sub(self, rhs: Matrix) -> Matrix {
        let negative_rhs : Matrix = rhs * -1.0;
        return self + negative_rhs;
    }
}

impl ops::SubAssign for Matrix {
    /// Subtracts and assigns matrices
    fn sub_assign(&mut self, rhs: Matrix) {
        *self = self.clone() - rhs;
    }
}

impl ops::Mul for Matrix {
    type Output = Matrix;

    /// Multiplies two matrices together. Abides by standard matrix multiplication rules
    fn mul(self, rhs: Matrix) -> Matrix {
        if self.columns != rhs.rows {
            panic!("Left hand columns must equal right hand rows!");
        }

        let common_size : usize = self.columns;

        let mut output : Matrix = Matrix::new(self.rows, rhs.columns);

        for output_row in 0..self.rows {
            for output_column in 0..rhs.columns {

                let mut a : Vec<f64> = Vec::with_capacity(common_size);
                for i in 0..common_size {
                    a.push(self[output_row][i]);
                }

                let mut b : Vec<f64> = Vec::with_capacity(common_size);
                for i in 0..common_size {
                    b.push(rhs[i][output_column]);
                }

                output.set_value(output_row, output_column, Matrix::inner_product(a, b));
            }
        }

        return output;
    }

}

impl ops::Mul<f64> for Matrix {
    type Output = Matrix;

    /// Scales this matrix by rhs
    fn mul(self, rhs: f64) -> Matrix {
        let mut output : Matrix = Matrix::new(self.rows, self.columns);

        for row_index in 0..self.rows {
            for column_index in 0..self.columns {
                let value : f64 = self[row_index][column_index] * rhs;
                output.set_value(row_index, column_index, value);
            }
        }

        return output;
    }
}

impl ops::Mul<Matrix> for f64 {
    type Output = Matrix;

    /// Scales rhs matrix by self
    fn mul(self, rhs: Matrix) -> Matrix {
        return rhs * self;
    }
}

impl ops::MulAssign for Matrix {
    /// Multiplies and assigns matrices
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl ops::MulAssign<f64> for Matrix {
    /// Scales and assigns this matrix
    fn mul_assign(&mut self, rhs: f64) {
        *self = self.clone() * rhs;
    }
}

impl cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.columns != other.columns {
            return false;
        }

        for row in 0..self.rows {
            for column in 0..self.columns {
                if self[row][column] != other[row][column] {
                    return false;
                }
            }
        }

        return true;
    }

    fn ne(&self, other: &Self) -> bool {
        return !(self == other);
    }
}

impl ops::Index<usize> for Matrix {
    type Output = Vec<f64>;

    /// Grabs the indicated row of the matrix. Can then index that row to get a value, ie Matrix\[row\]\[column\]
    fn index(&self, index: usize) -> &Self::Output {
        return self.matrix[index].as_ref();
    }
}