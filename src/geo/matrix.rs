use std::ops::{Add, Div, Index, IndexMut, Mul};

use super::{point::Point, ray::Ray, vector::Vector};

pub enum Rotation {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.; cols]; rows],
        }
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        Matrix {
            rows: 4,
            cols: 4,
            data: vec![
                vec![1., 0., 0., x],
                vec![0., 1., 0., y],
                vec![0., 0., 1., z],
                vec![0., 0., 0., 1.],
            ],
        }
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        Matrix {
            rows: 4,
            cols: 4,
            data: vec![
                vec![x, 0., 0., 0.],
                vec![0., y, 0., 0.],
                vec![0., 0., z, 0.],
                vec![0., 0., 0., 1.],
            ],
        }
    }

    pub fn rotation(angle: f64, axis: Rotation) -> Matrix {
        let mut matrix = Matrix::new(4, 4);
        let cosr = angle.cos();
        let sinr = angle.sin();
        match axis {
            Rotation::X => {
                matrix[1][1] = cosr;
                matrix[2][2] = cosr;
                matrix[1][2] = -sinr;
                matrix[2][1] = sinr;
            }
            Rotation::Y => {
                matrix[0][0] = cosr;
                matrix[2][2] = cosr;
                matrix[0][2] = sinr;
                matrix[2][0] = -sinr;
            }
            Rotation::Z => {
                matrix[0][0] = cosr;
                matrix[1][1] = cosr;
                matrix[0][1] = -sinr;
                matrix[1][0] = sinr;
            }
        }
        matrix
    }

    pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
        Matrix {
            rows: 4,
            cols: 4,
            data: vec![
                vec![1., x_y, x_z, 0.],
                vec![y_x, 1., y_z, 0.],
                vec![z_x, z_y, 1., 0.],
                vec![0., 0., 0., 1.],
            ],
        }
    }

    pub fn from_vec(data: Vec<Vec<f64>>) -> Matrix {
        let rows = data.len();
        let cols = data[0].len();
        Matrix { rows, cols, data }
    }

    pub fn to_vec(&self) -> Vec<Vec<f64>> {
        self.data.clone()
    }

    pub fn transpose(&self) -> Result<Matrix, String> {
        if self.rows != self.cols {
            return Err("Matrix is not square".to_string());
        }
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[j][i] = self[i][j];
            }
        }
        Ok(result)
    }

    pub fn determinant(&self) -> Result<f64, String> {
        if self.rows != self.cols {
            return Err("Matrix is not square".to_string());
        }

        Self::sub_determinant(self.to_vec())
    }

    fn sub_determinant(matrix: Vec<Vec<f64>>) -> Result<f64, String> {
        if matrix.len() == 1 {
            return Ok(matrix[0][0]);
        }
        if matrix.len() == 2 {
            return Ok(matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]);
        }
        let mut result = 0.;
        for i in 0..matrix.len() {
            // this should slice the row and column
            let sub_matrix = Self::sub_matrix(&matrix, 0, i);
            let polarity = if i % 2 == 0 { 1. } else { -1. };
            result += matrix.first().expect("No last row")[i]
                * polarity
                * Self::sub_determinant(sub_matrix)?;
        }

        Ok(result)
    }

    fn sub_matrix(matrix: &Vec<Vec<f64>>, row: usize, col: usize) -> Vec<Vec<f64>> {
        matrix
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != row)
            .map(|(_, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(j, _)| *j != col)
                    .map(|(_, &val)| val)
                    .collect()
            })
            .collect()
    }

    fn cofactor(&self) -> Result<Matrix, String> {
        let mut result = Matrix::new(self.rows, self.cols);
        let mut polarities = String::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let polarity = if (i + j) % 2 == 0 { 1. } else { -1. };
                result[i][j] =
                    polarity * Self::sub_determinant(Self::sub_matrix(&self.to_vec(), i, j))?;
            }
        }
        dbg!(polarities);
        Ok(result)
    }

    pub fn inverse(&self) -> Result<Matrix, String> {
        let cofactor = self.cofactor()?.transpose()?;
        let det = self.determinant()?;
        Ok(cofactor / det)
    }

    pub fn round(&self, digits: usize) -> Matrix {
        fn round_to(val: f64, digits: usize) -> f64 {
            let factor = 10.0_f64.powi(digits as i32);
            (val * factor).round() / factor
        }
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .data
                .clone()
                .into_iter()
                .map(|row| row.into_iter().map(|val| round_to(val, 5)).collect())
                .collect(),
        }
    }
}

impl Index<usize> for Matrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.cols, rhs.rows);
        let mut result = Matrix::new(self.rows, rhs.cols);
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let mut sum = 0.;
                for k in 0..self.cols {
                    sum += self[i][k] * rhs[k][j];
                }
                result[i][j] = sum;
            }
        }
        result
    }
}

impl Div<f64> for Matrix {
    type Output = Matrix;

    fn div(self, rhs: f64) -> Matrix {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .data
                .into_iter()
                .map(|row| row.into_iter().map(|val| val / rhs).collect())
                .collect(),
        }
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        let x = self[0][0] * other.x + self[0][1] * other.y + self[0][2] * other.z;
        let y = self[1][0] * other.x + self[1][1] * other.y + self[1][2] * other.z;
        let z = self[2][0] * other.x + self[2][1] * other.y + self[2][2] * other.z;
        Point { x, y, z }
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        let x = self[0][0] * other.x + self[0][1] * other.y + self[0][2] * other.z;
        let y = self[1][0] * other.x + self[1][1] * other.y + self[1][2] * other.z;
        let z = self[2][0] * other.x + self[2][1] * other.y + self[2][2] * other.z;
        Vector { x, y, z }
    }
}

impl Mul<Ray> for Matrix {
    type Output = Ray;
    fn mul(self, ray: Ray) -> Ray {
        Ray::new(self.clone() * ray.origin, self * ray.direction)
    }
}

impl Add<Point> for Matrix {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        let x = self[0][0] + other.x;
        let y = self[1][0] + other.y;
        let z = self[2][0] + other.z;
        Point { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_2x2() {
        let mut matrix = Matrix::new(2, 2);
        matrix[0][0] = -3.;
        matrix[0][1] = 5.;
        matrix[1][0] = 1.;
        matrix[1][1] = 2.;
        assert_eq!(matrix[0][0], -3.);
        assert_eq!(matrix[0][1], 5.);
        assert_eq!(matrix[1][0], 1.);
        assert_eq!(matrix[1][1], 2.);
    }

    #[test]
    fn test_matrix_3x3() {
        let mut matrix = Matrix::new(3, 3);
        matrix[0][0] = 1.;
        matrix[0][1] = 2.;
        matrix[0][2] = 3.;
        matrix[1][0] = 4.;
        matrix[1][1] = 5.;
        matrix[1][2] = 6.;
        matrix[2][0] = 7.;
        matrix[2][1] = 8.;
        matrix[2][2] = 9.;
        assert_eq!(matrix[0][0], 1.);
        assert_eq!(matrix[0][1], 2.);
        assert_eq!(matrix[0][2], 3.);
        assert_eq!(matrix[1][0], 4.);
        assert_eq!(matrix[1][1], 5.);
        assert_eq!(matrix[1][2], 6.);
        assert_eq!(matrix[2][0], 7.);
        assert_eq!(matrix[2][1], 8.);
        assert_eq!(matrix[2][2], 9.);
    }

    #[test]
    fn test_matrix_equal() {
        let mut matrix = Matrix::new(2, 2);
        matrix[0][0] = -3.;
        matrix[0][1] = 5.;
        matrix[1][0] = 1.;
        matrix[1][1] = 2.;

        let mut matrix2 = Matrix::new(2, 2);
        matrix2[0][0] = -3.;
        matrix2[0][1] = 5.;
        matrix2[1][0] = 1.;
        matrix2[1][1] = 2.;

        let _ = matrix == matrix2;

        assert_eq!(matrix, matrix2);
    }

    #[test]
    fn test_matrix_mul() {
        let mut matrix = Matrix::new(2, 2);
        matrix[0][0] = -3.;
        matrix[0][1] = 5.;
        matrix[1][0] = 1.;
        matrix[1][1] = 2.;
        let mut matrix2 = Matrix::new(2, 2);
        matrix2[0][0] = 1.;
        matrix2[0][1] = 2.;
        matrix2[1][0] = 4.;
        matrix2[1][1] = 5.;
        let result = matrix * matrix2;
        assert_eq!(result[0][0], 17.);
        assert_eq!(result[0][1], 19.);
        assert_eq!(result[1][0], 9.);
        assert_eq!(result[1][1], 12.);
    }

    #[test]
    fn test_matrix_mul_3x3() {
        let matrix = Matrix::from_vec(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
        let matrix2 = Matrix::from_vec(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
        let result = matrix * matrix2;
        let result_vec = result.to_vec();

        let answer = vec![
            vec![30., 36., 42.],
            vec![66., 81., 96.],
            vec![102., 126., 150.],
        ];

        assert_eq!(result_vec, answer);
    }

    #[test]
    fn test_matrix_transpose() {
        let matrix = Matrix::from_vec(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
        let result = match matrix.transpose() {
            Ok(it) => it,
            _ => panic!("Should have succeeded"),
        };
        let result_vec = result.to_vec();
        let answer = vec![vec![1., 4., 7.], vec![2., 5., 8.], vec![3., 6., 9.]];
        assert_eq!(result_vec, answer);
    }

    #[test]
    fn test_determinant_2x2() {
        let matrix = Matrix::from_vec(vec![vec![1., 2.], vec![3., 4.]]);
        let result = matrix.determinant().expect("No determinant");
        assert_eq!(result, -2.);
    }

    #[test]
    fn test_determinant_3x3() {
        let matrix = Matrix::from_vec(vec![vec![1., 2., 3.], vec![-5., 8., -4.], vec![2., 6., 4.]]);
        let result = matrix.determinant().expect("No determinant");
        assert_eq!(result, -58.);
    }

    #[test]
    fn test_determinant_4x4() {
        let matrix = Matrix::from_vec(vec![
            vec![-5., 2., 6., -8.],
            vec![1., -5., 1., 8.],
            vec![7., 7., -6., -7.],
            vec![1., -3., 7., 4.],
        ]);
        let result = matrix.determinant().expect("No determinant");

        assert_eq!(result, 532.);
    }

    #[test]
    fn test_inverse_4x4() {
        let matrix = Matrix::from_vec(vec![
            vec![-5., 2., 6., -8.],
            vec![1., -5., 1., 8.],
            vec![7., 7., -6., -7.],
            vec![1., -3., 7., 4.],
        ]);

        let result = matrix.inverse().expect("No inverse").round(5);
        let answer = vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ];

        let answer = Matrix::from_vec(answer);
        assert_eq!(result, answer);
    }

    #[test]
    fn test_matrix_translate() {
        let matrix = Matrix::translation(1., 2., 3.);
        let result = matrix * Point::new(0., 0., 0.);
        assert_eq!(result, Point::new(1., 2., 3.));
    }
}
