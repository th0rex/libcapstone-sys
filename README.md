# libcapstone-sys

Complete rust bindings to the [capstone engine](https://github.com/aquynh/capstone). Currently only Linux is supported.
We also currently require a nightly version of Rust.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You'll need to have installed the capstone engine. The library should be in one of the default library directories when using Linux/OS X.

You can set the environment variable `CAPSTONE_INCLUDE_DIR` to the directory where the header files for the capstone engine are.

Then you can simply `git clone` this repository and `cd` into it.

### Building

To build this project simply execute `cargo build`.

## Running the tests

Simply execute `cargo test`.

## Built With

* [rust-bindgen](https://github.com/servo/rust-bindgen) - Generate Rust FFI bindings to C and C++ libraries.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
