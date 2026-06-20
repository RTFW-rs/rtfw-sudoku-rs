use criterion::{Criterion, criterion_group, criterion_main};
use rtfw_sudoku_rs::SudokuBoard;

fn bench_solve_easy(c: &mut Criterion) {
    let board = SudokuBoard::from_definition_str(
        "
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
",
    );
    c.bench_function("solve_greedy easy", |b| {
        b.iter(|| SudokuBoard::solve_greedy(&board))
    });
}

fn bench_solve_hard(c: &mut Criterion) {
    let board = SudokuBoard::from_definition_str(
        "
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
",
    );
    c.bench_function("solve_greedy hard", |b| {
        b.iter(|| SudokuBoard::solve_greedy(&board))
    });
}

criterion_group!(benches, bench_solve_easy, bench_solve_hard);
criterion_main!(benches);
