use crate::vec::Vec3i;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix {
    columns: usize,
    data: Vec<i64>,
}

impl Matrix {
    pub fn from_rows(rows: &[Vec3i]) -> Matrix {
        let size = rows.len() * 3;
        let data = rows.iter().fold(Vec::with_capacity(size), |mut acc, row| {
            acc.push(row.x);
            acc.push(row.y);
            acc.push(row.z);
            acc
        });
        Matrix { columns: 3, data }
    }

    pub fn column(values: &[i64]) -> Matrix {
        let data = values.iter().copied().collect();
        Matrix { columns: 1, data }
    }

    pub fn transpose(&self) -> Matrix {
        let columns = self.rows();
        let mut data = vec![0; self.data.len()];
        for (idx, value) in self.data.iter().enumerate() {
            let src_row = idx / self.columns;
            let src_column = idx % self.columns;
            data[src_column * columns + src_row] = *value;
        }
        Matrix { columns, data }
    }

    fn rows(&self) -> usize {
        self.data.len() / self.columns
    }

    pub fn determinant(&self) -> Option<i64> {
        let rows = self.rows();
        let columns = self.columns;
        if rows != columns {
            None
        } else {
            if self.columns == 1 {
                Some(self.data[0])
            } else {
                let mut determinant = 0;
                for row in 0..rows {
                    let mut data = vec![0; (rows - 1) * (columns - 1)];
                    for alt_row in 0..(rows - 1) {
                        for alt_column in 0..(columns - 1) {
                            let delta = if alt_row < row { 0 } else { 1 };
                            data[alt_row * (columns - 1) + alt_column] =
                                self.data[(alt_row + delta) * columns + alt_column + 1];
                        }
                    }
                    let matrix = Matrix {
                        columns: columns - 1,
                        data,
                    };
                    let det = matrix.determinant()?;
                    determinant +=
                        if row % 2 == 0 { 1 } else { -1 } * self.data[row * columns] * det;
                }
                Some(determinant)
            }
        }
    }

    pub fn adjoint(&self) -> Option<Matrix> {
        let rows = self.rows();
        let columns = self.columns;
        let mut data = vec![0; rows * columns];
        for row in 0..rows {
            for column in 0..columns {
                let mut adj = vec![0; (rows - 1) * (columns - 1)];
                for row_alt in 0..(rows - 1) {
                    for column_alt in 0..(columns - 1) {
                        let row_idx = row_alt + if row_alt < row { 0 } else { 1 };
                        let column_idx = column_alt + if column_alt < column { 0 } else { 1 };
                        adj[row_alt * (columns - 1) + column_alt] =
                            self.data[row_idx * columns + column_idx];
                    }
                }
                let minor = Matrix {
                    columns: columns - 1,
                    data: adj,
                };
                data[row * columns + column] +=
                    if (column + row) % 2 == 0 { 1 } else { -1 } * minor.determinant()?;
            }
        }
        Some(Matrix { columns, data }.transpose())
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.columns, rhs.rows());
        let rows = self.rows();
        let columns = rhs.columns;
        let mut data = vec![0; rows * columns];
        for row in 0..rows {
            for column in 0..columns {
                data[row * columns + column] = (0..self.columns)
                    .map(|idx| {
                        self.data[row * self.columns + idx] * rhs.data[idx * rhs.columns + column]
                    })
                    .sum();
            }
        }
        Matrix { columns, data }
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose() {
        let vs = [1, 2, 3];
        let m = Matrix::column(&vs);
        let t = m.transpose();
        assert_eq!(t.columns, 3);
        assert_eq!(t.data[0], 1);
        assert_eq!(t.data[1], 2);
        assert_eq!(t.data[2], 3);
    }

    #[test]
    fn mul() {
        let vs = [1, 2, 3];
        let m = Matrix::column(&vs);
        let t = m.transpose();
        let mult = t * m;
        assert_eq!(mult.columns, 1);
        assert_eq!(mult.data.len(), 1);
        assert_eq!(mult.data[0], 14);
    }

    #[test]
    fn det() {
        let data = vec![1, 0, 4, -6, 2, 5, 0, 3, -1, 2, 3, 5, 2, 1, -2, 3];
        let matrix = Matrix { columns: 4, data };
        assert_eq!(matrix.determinant().unwrap(), 318);
    }

    #[test]
    fn adj() {
        let data = vec![1, 5, 2, 0, -1, 2, 0, 0, 1];
        let matrix = Matrix { columns: 3, data };
        let adj = matrix.adjoint().unwrap();

        assert_eq!(adj.columns, 3);
        assert_eq!(adj.data.len(), 9);

        assert_eq!(adj.data[0], -1);
        assert_eq!(adj.data[1], -5);
        assert_eq!(adj.data[2], 12);

        assert_eq!(adj.data[3], 0);
        assert_eq!(adj.data[4], 1);
        assert_eq!(adj.data[5], -2);

        assert_eq!(adj.data[6], 0);
        assert_eq!(adj.data[7], 0);
        assert_eq!(adj.data[8], -1);
    }

    #[test]
    fn mult_rect() {
        let m1 = Matrix {
            columns: 3,
            data: vec![1, 2, 3, 4, 5, 6],
        };
        let m2 = Matrix {
            columns: 2,
            data: vec![10, 11, 20, 21, 30, 31],
        };
        let m = m1 * m2;

        assert_eq!(m.columns, 2);
        assert_eq!(m.data.len(), 4);

        assert_eq!(m.data[0], 140);
        assert_eq!(m.data[1], 146);
        assert_eq!(m.data[2], 320);
        assert_eq!(m.data[3], 335);
    }
}
