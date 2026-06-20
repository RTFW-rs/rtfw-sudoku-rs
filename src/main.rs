use rtfw_sudoku_rs::SudokuBoard;

fn main() {
    env_logger::init();
    let raw_board = "
|  1|   |   |
|   |  1|   |
|   |   |2  |
|---|---|---|
|   |   | 1 |
|   |   |   |
|   |   |   |
|---|---|---|
|   |   |  1|
|   |   |   |
|   |   |   |
";
    let sudoku = SudokuBoard::from_definition_str(raw_board);
    let solved = SudokuBoard::solve_greedy(&sudoku).unwrap();
    println!("{}", sudoku);
    println!("{}", solved);
}
