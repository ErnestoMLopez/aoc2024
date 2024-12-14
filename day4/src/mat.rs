pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    matrix: Vec<T>,
}

pub struct MatrixRowsIterator<'a, T: 'a + Clone> {
    origin: &'a Matrix<T>,
    current_row: usize,
}

pub struct MatrixColsIterator<'a, T: 'a + Clone> {
    origin: &'a Matrix<T>,
    current_col: usize,
}

pub struct MatrixUwDiagIterator<'a, T: 'a + Clone> {
    origin: &'a Matrix<T>,
    current_row: usize,
    current_col: usize,
}

pub struct MatrixDwDiagIterator<'a, T: 'a + Clone> {
    origin: &'a Matrix<T>,
    current_row: usize,
    current_col: usize,
}

impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize, matrix: Vec<T>) -> Self {
        Matrix { rows, cols, matrix }
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.matrix[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.matrix[row * self.cols + col] = value;
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl<'a, T: 'a + Clone> Matrix<T> {
    pub fn iter_rows(&'a self) -> MatrixRowsIterator<'a, T> {
        MatrixRowsIterator {
            origin: self,
            current_row: 0,
        }
    }

    pub fn iter_cols(&'a self) -> MatrixColsIterator<'a, T> {
        MatrixColsIterator {
            origin: self,
            current_col: 0,
        }
    }

    pub fn iter_uw_diag(&'a self) -> MatrixUwDiagIterator<'a, T> {
        MatrixUwDiagIterator {
            origin: self,
            current_row: 0,
            current_col: 0,
        }
    }

    pub fn iter_dw_diag(&'a self) -> MatrixDwDiagIterator<'a, T> {
        MatrixDwDiagIterator {
            origin: self,
            current_row: 0,
            current_col: 0,
        }
    }
}

impl Matrix<char> {
    pub fn from_string(input: &str) -> Self {
        let mut matrix = Vec::new();
        let mut rows = 0;
        let mut cols = 0;

        for line in input.lines() {
            cols = line.len();
            matrix.extend(line.chars());
            rows += 1;
        }

        Matrix { rows, cols, matrix }
    }
}

impl<'a, T: 'a + Clone> Iterator for MatrixRowsIterator<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.current_row >= self.origin.rows {
            None
        } else {
            let start = self.current_row * self.origin.cols;
            let end = start + self.origin.cols;
            self.current_row += 1;
            Some(self.origin.matrix[start..end].to_vec())
        }
    }
}

impl<'a, T: 'a + Clone> Iterator for MatrixColsIterator<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.current_col >= self.origin.cols {
            None
        } else {
            let mut column = Vec::new();
            self.origin
                .matrix
                .iter()
                .skip(self.current_col)
                .step_by(self.origin.cols)
                .for_each(|x| column.push(x.clone()));
            self.current_col += 1;
            Some(column)
        }
    }
}

impl<'a, T: 'a + Clone> Iterator for MatrixDwDiagIterator<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.current_row >= self.origin.rows {
            None
        } else {
            let mut diagonal = Vec::new();

            let rows = self.origin.rows;
            let cols = self.origin.cols;

            // Si la diagonal termina anticipadamente tengo que limitar el vector, para no pasarme
            // de los elementos que efectivamente corresponden a la diagonal. No veo otra forma que
            // calcular el índice del elemento final y cortar ahí.
            let remaining_rows = rows - self.current_row - 1;
            let remaining_cols = cols - self.current_col - 1;
            let remaining_elements = std::cmp::min(remaining_rows, remaining_cols);
            let end_row = self.current_row + remaining_elements;
            let end_col = self.current_col + remaining_elements;
            let elements = end_row * cols + end_col + 1;

            self.origin
                .matrix
                .iter()
                .take(elements)
                .skip(self.current_row * cols + self.current_col)
                .step_by(self.origin.cols + 1)
                .for_each(|x| diagonal.push(x.clone()));

            if self.current_row == 0 {
                self.current_col += 1;

                if self.current_col >= self.origin.cols {
                    self.current_col = 0;
                    self.current_row += 1;
                }
            } else {
                self.current_row += 1;
            }

            Some(diagonal)
        }
    }
}

impl<'a, T: 'a + Clone> Iterator for MatrixUwDiagIterator<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.current_row >= self.origin.rows {
            None
        } else {
            let mut diagonal = Vec::new();

            let rows = self.origin.rows;
            let cols = self.origin.cols;

            // Si la diagonal termina anticipadamente tengo que limitar el vector, para no pasarme
            // de los elementos que efectivamente corresponden a la diagonal. No veo otra forma que
            // calcular el índice del elemento final y cortar ahí.
            let remaining_rows = rows - self.current_row - 1;
            let remaining_cols = self.current_col;
            let remaining_elements = std::cmp::min(remaining_rows, remaining_cols);
            let end_row = self.current_row + remaining_elements;
            let end_col = self.current_col - remaining_elements;
            let elements = end_row * cols + end_col + 1;

            self.origin
                .matrix
                .iter()
                .take(elements)
                .skip(self.current_row * cols + self.current_col)
                .step_by(self.origin.cols - 1)
                .for_each(|x| diagonal.push(x.clone()));

            if self.current_row == 0 {
                self.current_col += 1;

                if self.current_col >= self.origin.cols {
                    self.current_col = self.origin.cols - 1;
                    self.current_row += 1;
                }
            } else {
                self.current_row += 1;
            }

            Some(diagonal)
        }
    }
}
