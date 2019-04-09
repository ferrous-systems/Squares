- requires nightly
- required JSON object: {"row": i32,"column": i32,"red": u8,"green": u8,"blue": u8}
  - colors 0-255, row and column 1 - your specified maximum
- send post request to http://localhost:8000/cell

- run with args <rows> <columns>
- usage of test
  - cargo run <row> <column> <red> <green> <blue>

- toggle fullscreen: space
- quit: esc 
