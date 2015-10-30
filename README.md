# rgmk #

Library for manipulating Game Maker Studio's "data.win" (GEN8) data files.

It was primarily designed for, and only tested on [UNDERTALE](http://undertale.com/).

## Usage ##
rgmk is written in [Rust](https://www.rust-lang.org/).
You will need to install and download the latest **nightly** version of Rust.

Once you have done that, you can build rgmk with `cargo build --release`.

## Tools ##
rgmk provides a few tools that you can find in the `target/release` folder.

These include
- `stringtool` A string manipulation tool
- `texturetool` A texture manipulation tool (to manipulate graphics)

## Library ##
rgmk can be used as a library in your own Rust project, to build your own custom tools.

Credits:
- https://github.com/panzi/cook-serve-hoomans/blob/master/fileformat.md for lots of useful information.
