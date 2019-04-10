
## Squares - drawing squares from http requests

The program generates a grid in the size of your choice. The squares can be colored
by sending a JSON object via POST requests containing the coordinates of the square
in the grid as well as RGB values.

# Requirements
- Rust nightly
- sdl2 library

# Usage
- run with args: \<rows\> \<columns\>
- toggle fullscreen: space
- clear grid: return
- quit: esc

- usage of squares_test:
  - colors 0-255, row and column 1 - your specified maximum
  - cargo run \<row\> \<column\> \<red\> \<green\> \<blue\>

# Squares_test
Squares_test sends JSON objects to Squares, when running locally.
- usage of squares_test:

  - cargo run \<row\> \<column\> \<red\> \<green\> \<blue\>
  - Allowed Values for the arguments:
    - colors: 0-255
    - row and column: 1 - max