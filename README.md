# SudokuSolver
The purpose of this project is to implement a Sudoku solving algorithm in different programming languages as a means of familirizing myself with those languages.

## Algorithm
The algorithm is as follows
### Setup
- Create an empty board where each square has all 9 possibilities (numbers 1-9)
- When a number is placed onto the board, that number is removed from the possibilities of the corresponding column, row, and box
### Basic Solving
- If a square has only one possibility, it becomes that possibility, removing that number fom the corresponding column/row/box
- If a possibility only exists in one square of a column/row/box, it becomes that possibility, removing that number fom the corresponding column/row/box
- Repeat until the board no longer changes (this will solve most easy/meduim level Sudoku)
- If the board is not yet solved, perform a deep check
### Deep Check
- If a possibility can only exist on one column/row of a box, removes that possibility from the same column/row in other corresponding boxes
- Repeat basic solving and deep check until the board no longer changes (this will solve most hard level Sudoku)
- If the board is still not solved, use the last resort
### Last Resort
- Select an unsolved square and change it to one of it's possibilities
- Recurse into basic solving and deep checks until the board is either solved, or cannot be solved (indicated by an unsolved square having no possibilities left)
- If the board cannot be solved after choosing one of a square's possibilities, remove that possibility and try solving again
- Repeat until solved (will solve any remaining sudoku)
While this final step can technically solve any sudoku given enough time, it is extremely inefficient. Fortunately, it typically only needs to be used a few times for expert level Sudoku, and it can be optimized by choosing the square with the fewest possibilities (usually 2 or 3 after previous solving efforts).
