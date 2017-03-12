# libcapstone-sys

Complete rust bindings to the [capstone engine](https://github.com/aquynh/capstone). Currently only Linux and OS X is supported.
Building on windows should work, but I currently can't test that.
A nightly version of Rust is also required.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You'll need to have installed the capstone engine.

You can set the environment variable `CAPSTONE_INCLUDE_DIR` to the directory where the header files for the capstone engine are.
The environment variable `CAPSTONE_LIBRARY_DIR` specifies the library directory where capstone is installed.
These default to `/usr/include` and `/usr/lib` on Linux and OS X. 

On windows there are no default library and include directories so you *must* specify these environment variables.

Then you can simply `git clone` this repository and `cd` into it.

### Building

To build this project simply execute `cargo build`.

## Running the tests

Simply execute `cargo test`.

## Built With

* [rust-bindgen](https://github.com/servo/rust-bindgen) - Generate Rust FFI bindings to C and C++ libraries.

## Difference between this and [capstone-rs](https://github.com/richo/capstone-rs)
`capstone-rs` does not have complete bindings to the capstone engine. For example, `cs_detail` is completly missing and also many constants
are missing.

libcapstone-sys generates bindings directly from the capstone headers, so nothing is missing!

However that results in an not rust like API, for which [libcapstone](https://github.com/th0rex/libcapstone) exists.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
