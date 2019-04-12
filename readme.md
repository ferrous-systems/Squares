
## Squares - drawing squares from http requests

The program generates a grid in the size of your choice. The squares can be colored
by sending JSON objects via POST requests containing the coordinates of the square
in the grid as well as RGB values.

# Download
  Get the zip file from [Ferrous Systems - Squares](https://github.com/ferrous-systems/Squares/archive/master.zip) or clone the repository from `git@github.com:ferrous-systems/Squares.git`.


# Requirements
  Rust nightly
  ```
  $ curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
  ```
  sdl2 library
  [SDL Wiki](https://wiki.libsdl.org/Installation)

# Usage
To run the program:
```
$YourDirectory/squares/squares cargo run \<rows\> \<columns\>
```
Example:
```
$YourDirectory/squares/squares cargo run 4 6
```
produces a grid with 4 rows and 6 columns.

- toggle fullscreen: space
- clear grid: return
- quit: esc
- pause: b

# How to color squares:
To color squares send POST requests of the following format:
``` {"row":\<i32\>,"column":\<i32\>,"red":\<u8\>,"green":\<u8\>,"blue":\<u8\>}
```
Allowed values:
- colors: 0-255
- row and column: 1 - your specified maximum

Example with curl:
``` curl --request POST --data '{"row":3,"column":5,"red":250,"green":68,"blue":199}' http://localhost:8000/cell
```

Example with squares_test (only on localhost):
``` $YourDirectory/squares/squares_test cargo run \<row\> \<column\> \<red\> \<green\> \<blue\>
```

#
