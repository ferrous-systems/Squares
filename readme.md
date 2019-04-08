- requires nightly
- required JSON object: {"row": i32,"column": i32,"red": u8,"green": u8,"blue": u8}
  - colors 0-255, row and column 0-14
- send post request to http://localhost:8000/cell


-usage of test
  - cargo run <row> <columm> <red> <green> <blue>
