## About

Rust rewrite of [diablo4-string-parser](https://github.com/alkhdaniel/diablo-4-string-parser)

Currently supports `.stl` and `.aff` files.

Drag a folder of `.stl` or `.aff` files over the binary.

Outputs string_list.json or aff_list.json.

## UI

You can use the ui to select a folder to parse by clicking `file` in the top left of the window.

You can still drag a folder over the binary.

Added data viewers. Viewing large files like `stl` should be done in release. `cargo build --release`

#
![](media/demo.gif)
