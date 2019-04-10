## Squares - drawing squares receiving http requests

- requires nightly and sdl2
- run with args: \<rows\> \<columns\>, numbers can't be bigger then screen resolution. No error handling yet, it doesn't crash, but it does't draw either.
- toggle fullscreen: space
- quit: esc

- usage of squares_test:
  - colors 0-255, row and column 1 - your specified maximum
  - cargo run \<row\> \<column\> \<red\> \<green\> \<blue\>

- http requests:
  - send to http://localhost:8000/cell
  - required format of JSON object: {"row": i32,"column": i32,"red": u8,"green": u8,"blue": u8}
