pub struct Solution {}

impl Solution {
    fn get_diagonals(n: usize, row: usize, col: usize) -> (usize, usize) {
        (n - 1 + row - col, (n << 1) - 1 + row + col)
    }

    fn generate_solutions(n: usize, row: usize, queens: Vec<usize>, cols: Vec<bool>, diagonals: Vec<bool>, solutions: &mut i32) {
        if row == n {
            *solutions += 1;
            return;
        }
        for (col, &free_col) in cols.iter().enumerate() {
            let (pos_diag, neg_diag) = Self::get_diagonals(n, row, col);
            if free_col && diagonals[pos_diag] && diagonals[neg_diag] {
                let mut new_queens = queens.clone();
                new_queens.push(row * n + col);

                let mut new_cols = cols.clone();
                new_cols[col] = false;

                let mut new_diagonals = diagonals.clone();
                new_diagonals[pos_diag] = false;
                new_diagonals[neg_diag] = false;

                Self::generate_solutions(n, row + 1, new_queens, new_cols, new_diagonals, solutions);
            }
        }
    }

    pub fn total_n_queens(n: i32) -> i32 {
        let size = n as usize;
        let mut solutions = 0;
        Self::generate_solutions(size, 0, vec![], vec![true; size], vec![true; (size << 2) - 2], &mut solutions);
        solutions
    }
}