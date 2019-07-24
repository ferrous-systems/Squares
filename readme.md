![purple square logo](https://github.com/ferrous-systems/Squares/blob/master/example%20images/logo.png " ")
# Squares - coloring squares from http requests

The program generates a grid in the size of your choice. There are several ways to color the cells of the grid: coloring single cells, lines of cells or 8x8 grids of cells. The program accepts JSON objects via POST requests. 

The easiest way to get JSON objects is to write a program that serialises the data
structures found in the protocol section.


## Download
  Get the zip file from [Ferrous Systems - Squares](https://github.com/ferrous-systems/Squares/archive/master.zip) or clone the repository from `git@github.com:ferrous-systems/Squares.git`.


## Requirements
  Rust nightly
  ```
  $ curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
  ```
  sdl2 library
  [SDL Wiki](https://wiki.libsdl.org/Installation)


## Usage
To run the program:
```
$YourDirectory/squares/squares cargo run <rows> <columns>
```
Example:
```
$YourDirectory/squares/squares cargo run 4 6
```
produces a grid with 4 rows and 6 columns:
![clear grid](https://github.com/ferrous-systems/Squares/blob/master/example%20images/5.png " ")


## Controls
- toggle fullscreen: space
- clear grid: return
- quit: esc
- toggle pause: B


## How to color single cells:
To color single cells send POST requests of the following format to hostname/cell:
```
{"row":<i32>,"column":<i32>,"red":<u8>,"green":<u8>,"blue":<u8>}
```
Allowed values:
- colors: 0-255
- row and column: 0 - your specified maximum - 1

### Example with curl:

```
curl --request POST --data '{"row":2,"column":4,"red":250,"green":68,"blue":199}' http://localhost:8000/cell
```

Running this will change the color of the cell in row 2 and column 4 to pink.
![pink cell in row 2 and column 4](https://github.com/ferrous-systems/Squares/blob/master/example%20images/2.png " ")

Example with squares_test (only on localhost):
```
$YourDirectory/squares/squares_test cargo run <row> <column> <red> <green> <blue>
```
```
$YourDirectory/squares/squares_test cargo run 3 4 77 46 90
```

Running this will change the color of the cell in row 3 and column 4 to purple.
![pink cell in row 2 and column 4 and purple cell in row 3 and column 4](https://github.com/ferrous-systems/Squares/blob/master/example%20images/3.png " ")

### Protocol

```
 struct Cell {
    row: usize,
    column: usize,
    red: u8,
    green: u8,
    blue: u8,
}
```


## How to draw lines

To draw a line, send POST requests of the following format to hostname/line:

```
{"row":<i32>,"column":<i32>,"red":<u8>,"green":<u8>,"blue":<u8>,"direction":<i32>,"length":<i32>}
```

Allowed values:
- colors: 0-255
- row and column: 0 - your specified maximum - 1
- direction: 1 for vertical, 0 for horizontal
- length: Any length. If it is too long to fit, the cells that are out of range will not be drawn.

The row and column values mark the starting point of the line. The length is the length of the entire line.

### Example with curl

In a 6 by 12 grid, this command adds a vertical purple line, starting in row 1 column 5, with a length of 5 cells.

```
curl --request POST --data '{"row":1,"column":5,"red":77,"green":0,"blue":120,"direction":1,"length":5}}' http://localhost:8000/line
```
![vertical purple line, starting in row 1 column 5, with a length of 5 cells](https://github.com/ferrous-systems/Squares/blob/add-draw-lines/example%20images/6.png " ")

To add a horizontal line, that starts in the same coordinate, with a different shade of purple, we run this command:

```
curl --request POST --data '{"row":1,"column":5,"red":77,"green":0,"blue":90,"direction":0,"length":4}}' http://localhost:8000/line
```

![horizontal purple line, starting in row 1 column 5, with a length of 4 cells](https://github.com/ferrous-systems/Squares/blob/add-draw-lines/example%20images/7.png " ")

### Protocol

```
struct Line {
    row: i32,
    column: i32,
    red: u8,
    green: u8,
    blue: u8,
    direction: i32,
    length: i32,
}
```


## How to color several cells at once

To draw an 8x8 grid of pixels at once, serialize a `struct ApiGrid` to a JSON object and send it as a POST request to hostname/grid. You don't want to type this JSON object by hand.

The struct has the following fields:

- zero_row: The row value of the projected grid, were the 0 row of your 8x8 grid will be.
- zero_column: The column value of the projected grid, were the 0 column of your 8x8 grid will be.
- api_grid: An array of the following type [[RGB; 8]; 8].

### Protocol

```
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

struct ApiGrid {
    zero_row: i32,
    zero_column: i32,
    api_grid: [[RGB; 8]; 8],
}
```

## Intervention
The program can be intervened by sending GET requests.

```
http://localhost:8000/intervention/true
or
http://localhost:8000/intervention/false
```
Sending a GET request containing `true` will pause the animation with a checker board screen. Sending `false` resets the grid and the animation continues from a blank screen.


![checker board](https://github.com/ferrous-systems/Squares/blob/master/example%20images/4.png " ")


This can be used to signal viewers that something else will happen on the screen, or to just reset the grid. Hitting `b` on the computer, the program is running on,  will pause the animation without clearing the screen. Hitting `return` will clear the screen without pausing the animation.  
