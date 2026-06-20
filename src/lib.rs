use std::fmt;
use std::io::Write;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Clone)]
pub struct SudokuBoard {
    cells: Vec<Option<u8>>,
}

impl SudokuBoard {
    pub fn new_empty() -> Self {
        Self {
            cells: vec![None; 9 * 9],
        }
    }

    pub fn from_definition_str(def: &str) -> Self {
        let mut cells = vec![];
        for line in def.lines() {
            let formatted = line.replace('|', "").replace('-', "");
            if formatted.is_empty() {
                continue;
            }

            for c in formatted.chars() {
                let digit = c.to_digit(10).map(|n| n as u8);
                cells.push(digit);
            }
        }

        Self { cells }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.cells[y * 9 + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Option<u8> {
        &mut self.cells[y * 9 + x]
    }

    pub fn solve_greedy(board: &SudokuBoard) -> Option<SudokuBoard> {
        let mut paths: u64 = 0;
        let unsolved = board.get_empty_cells().len();
        let total_paths: f64 = 9_f64.powi(unsolved as i32);
        println!("total paths to explore: {:.2e}", total_paths);
        let result = Self::solve_greedy_inner(board, &mut paths);
        eprintln!();
        result
    }

    fn format_thousands(n: u64) -> String {
        let s = n.to_string();
        let mut out = String::with_capacity(s.len() + s.len() / 3);
        for (i, c) in s.chars().enumerate() {
            if i != 0 && (s.len() - i) % 3 == 0 {
                out.push(',');
            }
            out.push(c);
        }
        out
    }

    fn solve_greedy_inner(board: &SudokuBoard, paths: &mut u64) -> Option<SudokuBoard> {
        *paths += 1;
        eprint!("\riterations: {}", Self::format_thousands(*paths));
        let _ = std::io::stderr().flush();

        if board.is_solved() {
            return Some(board.clone());
        }

        if !board.is_legal() {
            return None;
        }

        let unsolved = board.get_empty_cells();

        if let Some(cell_idx) = unsolved.first() {
            for digit in 1..=9 {
                let mut clone = board.clone();
                clone.cells[*cell_idx] = Some(digit);

                if let Some(res) = Self::solve_greedy_inner(&clone, paths) {
                    return Some(res);
                }
            }
        }

        None
    }

    fn get_empty_cells(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, d)| d.is_none())
            .map(|(i, _)| i)
            .collect()
    }

    pub fn is_solved(&self) -> bool {
        !self.cells.iter().any(|d| d.is_none()) && self.is_legal()
    }

    pub fn is_legal(&self) -> bool {
        self.check_columns() && self.check_rows() && self.check_boxes()
    }

    fn check_columns(&self) -> bool {
        for x in 0..9 {
            let mut digits_seen = vec![];
            for y in 0..9 {
                if let Some(digit) = self.get(x, y) {
                    if digits_seen.contains(&digit) {
                        return false;
                    }

                    digits_seen.push(digit);
                }
            }
        }

        true
    }

    fn check_rows(&self) -> bool {
        for y in 0..9 {
            let mut seen = vec![];
            for x in 0..9 {
                if let Some(digit) = self.get(x, y) {
                    if seen.contains(&digit) {
                        return false;
                    }

                    seen.push(digit);
                }
            }
        }

        true
    }

    fn check_boxes(&self) -> bool {
        for box_y in 0..3 {
            for box_x in 0..3 {
                let mut seen = [false; 10];

                for local_y in 0..3 {
                    for local_x in 0..3 {
                        let real_y = box_y * 3 + local_y;
                        let real_x = box_x * 3 + local_x;

                        if let Some(n) = self.get(real_x, real_y).map(|d| d as usize) {
                            if seen[n] {
                                return false; // duplicate in this 3x3 box
                            }

                            seen[n] = true;
                        }
                    }
                }
            }
        }

        true
    }
}

impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sudoku = "\n|".to_string();
        for (i, value) in self.cells.iter().enumerate() {
            if i != 0 && i % 3 == 0 {
                sudoku.push('|');
            }

            if i != 0 && i % 9 == 0 {
                sudoku.push_str("\n|");
            }

            if i != 0 && i % 27 == 0 {
                sudoku.push_str("---|---|---|\n|");
            }

            sudoku.push(match value {
                Some(val) => char::from(b'0' + val),
                None => ' ',
            });
        }
        sudoku.push_str("|\n");

        write!(f, "{}", sudoku)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_def_empty() {
        let input = "
|   |   |   |
|   |   |   |
|   |   |   |
|---|---|---|
|   |   |   |
|   |   |   |
|   |   |   |
|---|---|---|
|   |   |   |
|   |   |   |
|   |   |   |
";
        let board = SudokuBoard::from_definition_str(input);
        assert_eq!(input, board.to_string());
        assert_eq!(true, board.is_legal());
        assert_eq!(false, board.is_solved());
    }

    #[test]
    fn test_from_almost_empty_def() {
        let input = "
|   |   |   |
|   | 1 |   |
|   |   |   |
|---|---|---|
|   |   |   |
|   | 5 |   |
|   |   |   |
|---|---|---|
|   |   |   |
|   | 9 |   |
|   |   |   |
";
        let board = SudokuBoard::from_definition_str(input);
        assert_eq!(input, board.to_string());
        assert_eq!(true, board.is_legal());
        assert_eq!(false, board.is_solved());
    }

    #[test]
    fn test_full_solved1() {
        let input = "
|534|678|912|
|672|195|348|
|198|342|567|
|---|---|---|
|859|761|423|
|426|853|791|
|713|924|856|
|---|---|---|
|961|537|284|
|287|419|635|
|345|286|179|
";
        let board = SudokuBoard::from_definition_str(input);
        assert_eq!(input, board.to_string());
        assert_eq!(true, board.is_legal());
        assert_eq!(true, board.is_solved());
    }

    #[test]
    fn test_full_solved2() {
        let input = "
|123|456|789|
|456|789|123|
|789|123|456|
|---|---|---|
|214|365|897|
|365|897|214|
|897|214|365|
|---|---|---|
|531|642|978|
|642|978|531|
|978|531|642|
";
        let board = SudokuBoard::from_definition_str(input);
        assert_eq!(input, board.to_string());
        assert_eq!(true, board.is_legal());
        assert_eq!(true, board.is_solved());
    }

    #[test]
    fn test_full_invalid1() {
        let input = "
|133|456|789|
|456|789|123|
|789|123|456|
|---|---|---|
|214|365|897|
|365|897|214|
|897|214|365|
|---|---|---|
|531|642|978|
|642|978|531|
|978|531|642|
";
        let board = SudokuBoard::from_definition_str(input);
        assert_eq!(input, board.to_string());
        assert_eq!(false, board.is_legal());
        assert_eq!(false, board.is_solved());
    }

    #[test]
    fn test_full_invalid2() {
        let input = "
|123|456|789|
|156|789|123|
|789|123|456|
|---|---|---|
|214|365|897|
|365|897|214|
|897|214|365|
|---|---|---|
|531|642|978|
|642|978|531|
|978|531|642|
";
        let board = SudokuBoard::from_definition_str(input);
        assert_eq!(input, board.to_string());
        assert_eq!(false, board.is_legal());
        assert_eq!(false, board.is_solved());
    }

    #[test]
    fn test_full_invalid3() {
        let input = "
|123|456|789|
|456|789|123|
|189|123|456|
|---|---|---|
|214|365|897|
|365|897|214|
|897|214|365|
|---|---|---|
|531|642|978|
|642|978|531|
|978|531|642|
";
        let board = SudokuBoard::from_definition_str(input);
        assert_eq!(input, board.to_string());
        assert_eq!(false, board.is_legal());
        assert_eq!(false, board.is_solved());
    }

    #[test]
    fn test_partial_valid1() {
        let input = "
|53 | 7 |   |
|6  |195|   |
| 98|   | 6 |
|---|---|---|
|8  | 6 |  3|
|4  |8 3|  1|
|7  | 2 |  6|
|---|---|---|
| 6 |   |28 |
|   |419|  5|
|   | 8 | 79|
";
        let board = SudokuBoard::from_definition_str(input);
        assert_eq!(input, board.to_string());
        assert_eq!(true, board.is_legal());
        assert_eq!(false, board.is_solved());
    }

    #[test]
    fn test_partial_valid2() {
        let input = "
| 2 | 6 |7 1|
|68 |7  | 9 |
|19 | 45|   |
|---|---|---|
|8  |1  |4  |
|  4|6 2|9  |
|  5|  3| 28|
|---|---|---|
|   |93 | 74|
| 4 | 5 | 36|
|7 3| 18|   |
";
        let board = SudokuBoard::from_definition_str(input);
        assert_eq!(input, board.to_string());
        assert_eq!(true, board.is_legal());
        assert_eq!(false, board.is_solved());
    }

    #[test]
    fn test_partial_invalid1() {
        let input = "
|53 | 7 | 3 |
|6  |195|   |
| 98|   | 6 |
|---|---|---|
|8  | 6 |  3|
|4  |8 3|  1|
|7  | 2 |  6|
|---|---|---|
| 6 |   |28 |
|   |419|  5|
|   | 8 | 79|
";
        let board = SudokuBoard::from_definition_str(input);
        assert_eq!(input, board.to_string());
        assert_eq!(false, board.is_legal());
        assert_eq!(false, board.is_solved());
    }

    #[test]
    fn test_partial_invalid2() {
        let input = "
|53 | 7 |   |
|6  |195|   |
|598|   | 6 |
|---|---|---|
|8  | 6 |  3|
|4  |8 3|  1|
|7  | 2 |  6|
|---|---|---|
| 6 |   |28 |
|   |419|  5|
|   | 8 | 79|
";
        let board = SudokuBoard::from_definition_str(input);
        assert_eq!(input, board.to_string());
        assert_eq!(false, board.is_legal());
        assert_eq!(false, board.is_solved());
    }

    #[test]
    fn test_partial_invalid3() {
        let input = "
| 2 | 6 |7 1|
|68 |7  | 9 |
|19 | 45|   |
|---|---|---|
|82 |1  |4  |
|  4|6 2|9  |
|  5|  3| 28|
|---|---|---|
|   |93 | 74|
| 4 | 5 | 36|
|7 3| 18|   |
";
        let board = SudokuBoard::from_definition_str(input);
        assert_eq!(input, board.to_string());
        assert_eq!(false, board.is_legal());
        assert_eq!(false, board.is_solved());
    }

    #[test]
    fn test_get_empty_cells_empty_board() {
        let board = SudokuBoard::new_empty();
        let empty = board.get_empty_cells();
        assert_eq!(81, empty.len());
        assert_eq!((0..81).collect::<Vec<usize>>(), empty);
    }

    #[test]
    fn test_get_empty_cells_full_board() {
        let input = "
|534|678|912|
|672|195|348|
|198|342|567|
|---|---|---|
|859|761|423|
|426|853|791|
|713|924|856|
|---|---|---|
|961|537|284|
|287|419|635|
|345|286|179|
";
        let board = SudokuBoard::from_definition_str(input);
        assert!(board.get_empty_cells().is_empty());
    }

    #[test]
    fn test_get_empty_cells_partial_board() {
        let input = "
|53 | 7 |   |
|6  |195|   |
| 98|   | 6 |
|---|---|---|
|8  | 6 |  3|
|4  |8 3|  1|
|7  | 2 |  6|
|---|---|---|
| 6 |   |28 |
|   |419|  5|
|   | 8 | 79|
";
        let board = SudokuBoard::from_definition_str(input);
        let empty = board.get_empty_cells();
        assert_eq!(51, empty.len());
        // Known empty cells
        assert!(empty.contains(&2)); // row 0, col 2: |53 |
        assert!(empty.contains(&18)); // row 2, col 0: | 98|
        assert!(empty.contains(&63)); // row 7, col 0: |   |
        // Known filled cells must not appear
        assert!(!empty.contains(&0)); // '5' at (0,0)
        assert!(!empty.contains(&4)); // '7' at (4,0)
        assert!(!empty.contains(&80)); // '9' at (8,8)
    }

    #[test]
    fn test_get_empty_cells_single_filled_cell() {
        let mut board = SudokuBoard::new_empty();
        *board.get_mut(4, 4) = Some(5);
        let empty = board.get_empty_cells();
        assert_eq!(80, empty.len());
        let center_idx = 4 * 9 + 4; // 40
        assert!(!empty.contains(&center_idx));
    }

    #[test]
    fn test_solve_greedy_already_solved() {
        let input = "
|534|678|912|
|672|195|348|
|198|342|567|
|---|---|---|
|859|761|423|
|426|853|791|
|713|924|856|
|---|---|---|
|961|537|284|
|287|419|635|
|345|286|179|
";
        let board = SudokuBoard::from_definition_str(input);
        let result = SudokuBoard::solve_greedy(&board);
        assert!(result.is_some());
        assert!(result.unwrap().is_solved());
    }

    #[test]
    fn test_solve_greedy_illegal_board_returns_none() {
        let input = "
|133|456|789|
|456|789|123|
|789|123|456|
|---|---|---|
|214|365|897|
|365|897|214|
|897|214|365|
|---|---|---|
|531|642|978|
|642|978|531|
|978|531|642|
";
        let board = SudokuBoard::from_definition_str(input);
        let result = SudokuBoard::solve_greedy(&board);
        assert!(result.is_none());
    }

    #[test]
    fn test_solve_greedy_partial_board() {
        let input = "
|53 | 7 |   |
|6  |195|   |
| 98|   | 6 |
|---|---|---|
|8  | 6 |  3|
|4  |8 3|  1|
|7  | 2 |  6|
|---|---|---|
| 6 |   |28 |
|   |419|  5|
|   | 8 | 79|
";
        let board = SudokuBoard::from_definition_str(input);
        let result = SudokuBoard::solve_greedy(&board);
        assert!(result.is_some());
        assert!(result.unwrap().is_solved());
    }
}
