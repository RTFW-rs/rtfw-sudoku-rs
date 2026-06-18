use rtfw_sudoku_rs::SudokuBoard;

fn main() {
    let raw_board = "
|   |89 |6  |
| 9 | 64| 3 |
| 14| 32| 9 |
|---|---|---|
|   |  3|  6|
|  6|   |   |
|   |689|4 1|
|---|---|---|
|9  |   |  5|
|  1|4  |96 |
| 53|  6|17 |
";
    let sudoku = SudokuBoard::from_definition_str(raw_board);
    let solved = SudokuBoard::solve_greedy(&sudoku).unwrap();
    println!("{}", sudoku);
    println!("{}", solved);
}
