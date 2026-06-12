# 🧩 rtfw-sudoku-rs

![build](https://github.com/RTFW-rs/rtfw-sudoku-rs/actions/workflows/rust.yml/badge.svg)

A simple [Sudoku](https://en.wikipedia.org/wiki/Sudoku) library from scratch (checker, solver, etc.)

For input, it expects a 9x9 grid of digits (or space character) in the following format:

```
Solved Sudoku:
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

Partially solved Sudoku:
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

Empty Sudoku:
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
```
